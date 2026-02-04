# Get version from .version file or use default
VERSION := `cat .version 2>/dev/null || echo "0.1.0"`
REGISTRY := env_var_or_default("REGISTRY", "ghcr.io")
REPO_OWNER := env_var_or_default("REPO_OWNER", "bettyblocks")

build: build-all
build-all:
	#!/usr/bin/env bash
	for working_directory in $(just index); do
		echo "--- Running builds in $working_directory ---"
		(cd $working_directory && just build)
	done

propagate-justfiles:
	#!/usr/bin/env bash
	for working_directory in $(just index); do
		echo "--- Copying Justfile to $working_directory ---"
		(cp workspace-justfile "$working_directory/Justfile")
	done

index:
	#!/usr/bin/env bash
	find functions -type f -name "Cargo.toml" -exec dirname {} \;

