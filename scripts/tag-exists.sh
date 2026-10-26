#!/usr/bin/env bash
#
# Report whether <registry>/<repository>:<tag> already exists, through the
# registry's v2 API: a manifest HEAD with OCI and Docker manifest Accept
# headers. A Bearer challenge is answered with a pull token from the realm it
# names, requested with REGISTRY_USERNAME/REGISTRY_PASSWORD when set
# (anonymously otherwise).
#
# Exit status: 0 = the tag exists, 1 = it does not (404, also for an unknown
# repository), 2 = anything else (auth, network, unexpected status). Callers
# must treat 2 as a failure, never as "absent", or a tag could be overwritten.
#
# Usage: ./scripts/tag-exists.sh <registry> <repository> <tag>

set -euo pipefail

if [ "$#" -ne 3 ]; then
  echo "usage: $0 <registry> <repository> <tag>" >&2
  exit 2
fi

registry="$1" repository="$2" tag="$3"
url="https://${registry}/v2/${repository}/manifests/${tag}"
accept="application/vnd.oci.image.manifest.v1+json, application/vnd.oci.image.index.v1+json, application/vnd.docker.distribution.manifest.v2+json, application/vnd.docker.distribution.manifest.list.v2+json"

headers="$(mktemp)"
trap 'rm -f "$headers"' EXIT

# Credentials go through a curl config on stdin, never on the command line.
curl_auth() {
  if [ -n "${REGISTRY_USERNAME:-}" ]; then
    printf 'user = "%s:%s"\n' "$REGISTRY_USERNAME" "${REGISTRY_PASSWORD:-}" | curl -K - "$@"
  else
    curl "$@"
  fi
}

head_manifest() {
  curl -sS --retry 3 -o /dev/null -D "$headers" -w '%{http_code}' -I -H "Accept: ${accept}" "$@" "$url"
}

status="$(head_manifest)" || { echo "request to ${url} failed" >&2; exit 2; }

if [ "$status" = "401" ]; then
  challenge="$(grep -i '^www-authenticate:' "$headers" | tr -d '\r' | head -n1)"
  realm="$(printf '%s' "$challenge" | sed -nE 's/.*realm="([^"]*)".*/\1/p')"
  service="$(printf '%s' "$challenge" | sed -nE 's/.*service="([^"]*)".*/\1/p')"
  if [ -z "$realm" ] || ! printf '%s' "$challenge" | grep -qi 'bearer'; then
    echo "unexpected auth challenge from ${registry}: ${challenge}" >&2
    exit 2
  fi
  token="$(curl_auth -fsS --retry 3 -G "$realm" \
    --data-urlencode "service=${service}" \
    --data-urlencode "scope=repository:${repository}:pull" |
    jq -r '.access_token // .token // empty')" || { echo "token request to ${realm} failed" >&2; exit 2; }
  if [ -z "$token" ]; then
    echo "no token in the response from ${realm}" >&2
    exit 2
  fi
  status="$(head_manifest -H "Authorization: Bearer ${token}")" || { echo "request to ${url} failed" >&2; exit 2; }
fi

case "$status" in
  200) exit 0 ;;
  404) exit 1 ;;
  *)
    echo "unexpected HTTP ${status} for ${registry}/${repository}:${tag}" >&2
    exit 2
    ;;
esac
