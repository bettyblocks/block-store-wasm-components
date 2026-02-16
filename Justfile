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

integration-test:
	deno install
	deno fmt --check
	deno lint
	deno task test

run-just-command-all command_name: propagate-workspace-justfiles
	#!/usr/bin/env bash
	set -euo pipefail
	for working_directory in $(just index); do
		echo "--- Running {{ command_name }}s in $working_directory ---"
		(cd $working_directory && just {{ command_name }})
	done

index:
	find functions -type f -name "Cargo.toml" -exec dirname {} \;
