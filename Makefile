# For non-musl, use: armv7-unknown-linux-gnueabihf
#  TARGET ?= armv7-unknown-linux-musleabihf
TARGET ?= armv7-unknown-linux-gnueabihf

DEVICE_IP ?= '10.11.99.1'
DEVICE_HOST ?= root@$(DEVICE_IP)


.PHONY: deploy run

.cargo/config:
	wget https://raw.githubusercontent.com/canselcik/libremarkable/master/gen_cargo_config.py
	./gen_cargo_config.py
	rm gen_cargo_config.py

gomarkable: .cargo/config
	cargo build --release --target=$(TARGET)

test:
	# Notice we aren't using the armv7 target here
	cargo test

deploy: gomarkable
	ssh $(DEVICE_HOST) 'killall -q -9 gomarkable || true; systemctl stop xochitl remux || true'
	scp ./target/$(TARGET)/release/gomarkable $(DEVICE_HOST):
	ssh $(DEVICE_HOST) 'RUST_BACKTRACE=1 RUST_LOG=debug ./gomarkable'
run:
	ssh $(DEVICE_HOST) 'killall -q -9 gomarkable || true; systemctl stop xochitl remux || true'
	ssh $(DEVICE_HOST) './gomarkable'

