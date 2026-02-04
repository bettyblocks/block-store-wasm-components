# Get version from .version file or use default
VERSION := `cat .version 2>/dev/null || echo "0.1.0"`
REGISTRY := env_var_or_default("REGISTRY", "ghcr.io")
REPO_OWNER := env_var_or_default("REPO_OWNER", "bettyblocks")

build-all:
	#!/usr/bin/env bash
	for working_directory in $(just index); do
		echo "--- Running builds in $working_directory ---"
		(cd $working_directory && just build)
	done

build: build-all

index:
	#!/usr/bin/env bash
	find functions -type f -name "Cargo.toml" -exec dirname {} \;

