# For non-musl, use: armv7-unknown-linux-gnueabihf
#  TARGET ?= armv7-unknown-linux-musleabihf
TARGET ?= armv7-unknown-linux-gnueabihf

DEVICE_IP ?= 'remarkable'
DEVICE_HOST ?= root@$(DEVICE_IP)


.PHONY: deploy run build install-draft

.cargo/config:
	wget https://raw.githubusercontent.com/canselcik/libremarkable/master/gen_cargo_config.py
	./gen_cargo_config.py
	rm gen_cargo_config.py

./target/$(TARGET)/debug/gomarkable: .cargo/config src/*.rs build

test:
	# Notice we aren't using the armv7 target here
	cargo test
	
install-draft: gomarkable.draft
	ssh $(DEVICE_HOST) 'mkdir -p /home/root/.config/draft'
	scp gomarkable.draft $(DEVICE_HOST):/home/root/.config/draft/gomarkable.draft

build:
	cargo build --target=$(TARGET)

deploy: ./target/$(TARGET)/debug/gomarkable
	ssh $(DEVICE_HOST) 'killall -q -9 gomarkable || true; systemctl stop xochitl remux || true'
	scp ./target/$(TARGET)/debug/gomarkable $(DEVICE_HOST):
	ssh $(DEVICE_HOST) 'RUST_BACKTRACE=1 RUST_LOG=debug ./gomarkable'
run:
	ssh $(DEVICE_HOST) 'killall -q -9 gomarkable || true; systemctl stop xochitl remux || true'
	ssh $(DEVICE_HOST) './gomarkable'

