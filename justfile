#!/usr/bin/env just --justfile

package_name := `sed -En 's/name[[:space:]]*=[[:space:]]*"([^"]+)"/\1/p' Cargo.toml | head -1`
package_version := `sed -En 's/version[[:space:]]*=[[:space:]]*"([^"]+)"/\1/p' Cargo.toml | head -1`

default: fmt lint tests

test TEST:
	cargo test {{TEST}}

tests:
	sudo env "PATH=$PATH" cargo test --workspace -- --test-threads=1

run-root:
	sudo env "PATH=$PATH" cargo run

bench:
	cargo bench

lint:
	cargo clippy

fmt:
    cargo +nightly fmt --all

# pip3 install maturin
maturin:
    maturin build --release --strip --manylinux off -i /usr/bin/python3.9

create-venv:
    rm -rf venv && pip3 install virtualenv && virtualenv venv

venv:create-venv
  source ./venv/bin/activate; pip3 install maturin

docker:
    rm -rf docker_out
    docker build . --tag temp:1.0
    docker create --name temp temp:1.0
    docker cp temp:/build/target docker_out
    docker rm temp

run USER:
    chmod +x ./scripts/run.sh
    ./scripts/run.sh {{USER}}

build USER:
    chmod +x ./scripts/build.sh
    ./scripts/build.sh {{USER}}

develop:
  maturin develop --release

clean:
	cargo clean
	find . -type f -name "*.orig" -exec rm {} \;
	find . -type f -name "*.bk" -exec rm {} \;
	find . -type f -name ".*~" -exec rm {} \;