
image-raspberry-armv6: Dockerfile.raspberry-armv6
	docker build -f Dockerfile.raspberry-armv6 -t raspberry-armv6-rust:bullseye .

build-raspberry-armv6-debug:
	docker run -it --rm -v ${PWD}/..:/tmp/victron raspberry-armv6-rust:bullseye \
		bash -c "cd /tmp/victron/vedirect-prom && /root/.cargo/bin/cargo build --target=arm-unknown-linux-gnueabihf"

build-raspberry-armv6-release:
	docker run -it --rm -v ${PWD}/..:/tmp/victron raspberry-armv6-rust:bullseye \
		bash -c "cd /tmp/victron/vedirect-prom && /root/.cargo/bin/cargo build --target=arm-unknown-linux-gnueabihf --release"

image-raspberry-armv7: Dockerfile.raspberry-armv7
	docker build -f Dockerfile.raspberry-armv7 -t raspberry-armv7-rust:bullseye .

build-raspberry-armv7-debug:
	docker run -it --rm -v ${PWD}/..:/tmp/victron raspberry-armv7-rust:bullseye \
		bash -c "cd /tmp/victron/vedirect-prom && /root/.cargo/bin/cargo build --target=armv7-unknown-linux-gnueabihf"

build-raspberry-armv7-release:
	docker run -it --rm -v ${PWD}/..:/tmp/victron raspberry-armv7-rust:bullseye \
		bash -c "cd /tmp/victron/vedirect-prom && /root/.cargo/bin/cargo build --target=armv7-unknown-linux-gnueabihf --release"

clean:
	docker image rm raspberry-armv6-rust:bullseye raspberry-armv7-rust:bullseye
