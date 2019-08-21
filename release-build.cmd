cargo b --release
mkdir release > NUL 2>&1
copy target\release\envoy-dummy-stats.exe release\envoy-dummy-stats.exe