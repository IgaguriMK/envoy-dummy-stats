cargo b --release
docker run --rm -it -v %CD%:/home/rust/src ekidd/rust-musl-builder cargo build --release

mkdir release > NUL 2>&1

copy target\release\envoy-dummy-stats.exe release\envoy-dummy-stats.exe
copy target\x86_64-unknown-linux-musl\release\envoy-dummy-stats release\envoy-dummy-stats