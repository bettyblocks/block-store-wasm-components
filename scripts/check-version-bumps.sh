#!/usr/bin/env bash
#
# CI guard: every component whose code or WIT changed vs the base branch must
# also have its WIT package version bumped. Any increase (major, minor or patch)
# is enough; a downgrade fails. A component with no version on the base is new
# and may start at any version.
#
# Every PR targets main, and main publishes to production, so unlike
# native-wasm-components there is no branch on which a version may be reused.
#
# For each component we compare the version in its `package <ns>:<name>@X.Y.Z;`
# declaration in wit/world.wit against the same file on the base branch. A
# component is "changed" when any of these (tracked) files differ from the base:
#   - <component>/src/**, <component>/Cargo.toml, <component>/build.rs,
#     <component>/wit/** (excluding wit/deps/**)
#
# A component root is the parent of its wit/ directory
# (functions/<name>/<version>/).
#
# Shared files at the repository root never trigger the requirement: build.rs
# (the build script every component points at), Cargo.toml, Cargo.lock,
# wkg-config.toml, Justfile. Changing them would otherwise force a bump of
# every component. Neither do tests/, wkg.lock, wit/deps/ or target/.
#
# Usage: ./scripts/check-version-bumps.sh [base-ref]   (default: $GITHUB_BASE_REF or main)

set -euo pipefail
shopt -s nullglob

BASE_REF="${1:-${GITHUB_BASE_REF:-main}}"

main() {
  # Make the base ref available (no-op if already fetched, e.g. local dev).
  git fetch -q origin "$BASE_REF" 2>/dev/null || true

  if git rev-parse --verify -q "origin/${BASE_REF}^{commit}" >/dev/null; then
    BASE="origin/${BASE_REF}"
  elif git rev-parse --verify -q "${BASE_REF}^{commit}" >/dev/null; then
    BASE="${BASE_REF}"
  else
    echo "::error::base ref '${BASE_REF}' not found (tried origin/${BASE_REF} and ${BASE_REF})"
    exit 1
  fi

  echo "Comparing against base: ${BASE}"

  changed="$(git diff --name-only "${BASE}...HEAD")"
  if [ -z "$changed" ]; then
    echo "No changes vs base. Nothing to check."
    exit 0
  fi

  errors=()

  while IFS= read -r world; do
    [ -z "$world" ] && continue
    dir="$(dirname "$(dirname "$world")")"
    dir_re="$(printf '%s' "$dir" | sed 's/[.[\*^$()+?{}|]/\\&/g')"
    require_bump "$dir" "$world" \
      "^${dir_re}/(src/|build\.rs$|Cargo\.toml$|wit/)" \
      "^${dir_re}/wit/deps/"
  done < <(find functions -name world.wit -not -path '*/wit/deps/*' 2>/dev/null | sort)

  echo
  if [ "${#errors[@]}" -gt 0 ]; then
    echo "::error::Version bump required — the following components changed without a correct version bump:"
    for e in "${errors[@]}"; do echo "  • ${e}"; done
    exit 1
  fi
  echo "All changed components have a version bump. ✅"
}

# Extract the version that follows '@' in the first `package ...;` line (reads stdin).
version_of() {
  grep -m1 -E '^package ' | sed -nE 's/^package[^@]*@([^;[:space:]]+).*/\1/p'
}

# require_bump <label> <version-file> <include-ERE> [<exclude-ERE>]
require_bump() {
  local label="$1" vfile="$2" include="$3" exclude="${4:-}"
  local hits
  hits="$(printf '%s\n' "$changed" | grep -E "$include" || true)"
  [ -n "$exclude" ] && hits="$(printf '%s\n' "$hits" | grep -vE "$exclude" || true)"
  hits="$(printf '%s\n' "$hits" | grep -v '^[[:space:]]*$' || true)"
  [ -z "$hits" ] && return 0 # nothing relevant to this component changed

  local cur base
  cur="$(version_of <"$vfile" 2>/dev/null || true)"
  base="$(git show "${BASE}:${vfile}" 2>/dev/null | version_of || true)"

  if [ -z "$base" ]; then
    echo "🆕 ${label}: new component (no version on base) — OK"
    return 0
  fi
  if [ -z "$cur" ]; then
    errors+=("${label} — could not read a version from ${vfile}")
    echo "❌ ${label}: no version found in ${vfile}"
    return 0
  fi
  if [ "$cur" = "$base" ]; then
    errors+=("${label} — changed but version NOT bumped (still @${cur}); bump the version in ${vfile}")
    echo "❌ ${label}: changed, version still @${cur}"
    return 0
  fi

  # Any bump (major, minor or patch) is enough, as long as the version goes up.
  if [ "$(printf '%s\n%s\n' "$base" "$cur" | sort -V | tail -n1)" != "$cur" ]; then
    errors+=("${label} — @${base} → @${cur} is a downgrade; the version must go up")
    echo "❌ ${label}: @${base} → @${cur}, version went down"
  else
    echo "✅ ${label}: @${base} → @${cur}"
  fi
}

main "$@"
