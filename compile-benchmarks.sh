#!/bin/busybox ash
set -euo pipefail

image="ghcr.io/msrd0/abuild-armhf"
target="armv6-alpine-linux-musleabihf"

docker pull $image
docker run \
	--rm \
	-v "$(realpath "$(dirname "$0")")":/src \
	-w /src \
	$image \
	/bin/ash -euxo pipefail -c \
	'abuild-apk add --root $CBUILDROOT --arch $CTARGET libgcc musl-dev
	 abuild-apk add cargo rust-stdlib-armhf
	 cargo bench \
		--manifest-path benchmarks/Cargo.toml \
		--target-dir target/benchmarks \
		--target '$target' \
		--no-run'

find "$(dirname "$0")"/target/benchmarks/$target/release/deps \
	-type f -executable \
	-not -name '*.so'
