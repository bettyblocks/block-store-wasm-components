# Get version from .version file or use default
VERSION := `cat .version 2>/dev/null || echo "0.1.0"`
REGISTRY := env_var_or_default("REGISTRY", "ghcr.io")
REPO_OWNER := env_var_or_default("REPO_OWNER", "bettyblocks")

propagate-workspace-justfiles:
	#!/usr/bin/env bash
	for working_directory in $(just index); do
		echo "--- Copying Justfile to $working_directory ---"
		(cp workspace-justfile "$working_directory/Justfile")
	done

build: build-all
build-all:
	just run-just-command-all build

test: test-all
test-all:
	just run-just-command-all test

format: format-all
format-all:
	just run-just-command-all format

format-check: format-check-all
format-check-all:
	just run-just-command-all format-check

quality-check: quality-check-all
quality-check-all:
	just run-just-command-all quality-check

clean: clean-all
clean-all:
	just run-just-command-all clean

run-just-command-all command_name:
	#!/usr/bin/env bash
	for working_directory in $(just index); do
		echo "--- Running {{ command_name }}s in $working_directory ---"
		(cd $working_directory && just {{ command_name }})
	done

index:
	#!/usr/bin/env bash
	find functions -type f -name "Cargo.toml" -exec dirname {} \;

