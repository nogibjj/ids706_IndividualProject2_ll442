install:
	rustup update

build-up:		
	cargo build --release

test:
	cargo test

format:	
	cargo fmt

lint:
	cargo clippy

# container-lint:
# 	docker run --rm -i hadolint/hadolint < ./.devcontainer/Dockerfile

refactor: format lint

run:
	cargo run --bin sqlite_rust_project

deploy:
	cargo run --bin sqlite_rust_project

all: install lint test format refactor deploy