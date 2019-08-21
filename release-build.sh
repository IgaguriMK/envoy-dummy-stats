#! /bin/bash

docker run --rm -it -v ${PWD}:/home/rust/src ekidd/rust-musl-builder cargo build --release
mkdir -p release
cp target/x86_64-unknown-linux-musl/release/envoy-dummy-stats release/envoy-dummy-stats
