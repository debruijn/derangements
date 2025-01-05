.DEFAULT_GOAL := all

.PHONY: .pre-commit  # Check that pre-commit is installed
.pre-commit:
	@pre-commit -V || echo 'Please install pre-commit: https://pre-commit.com/'

.PHONY: install  # Install the package, dependencies, and pre-commit for local development
install: .pre-commit
	cargo build
	pre-commit install --install-hooks

.PHONY: format
format:
	cargo fmt

.PHONY: lint
lint:
	cargo fmt --version
	cargo fmt --all -- --check
	cargo clippy --version
	cargo clippy -- -D warnings -A incomplete_features -W clippy::dbg_macro -W clippy::print_stdout

.PHONY: test
test:
	cargo test

.PHONY: all
all: format test lint
