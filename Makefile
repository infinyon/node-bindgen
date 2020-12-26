RUSTV=stable

install_windows_on_mac:
	rustup target add x86_64-pc-windows-gnu
	brew install mingw-w64


build-windows:
	cargo build --target=x86_64-pc-windows-gnu


test:	test-derive test-examples
	

test-examples:
	make -C examples test	

test-derive:
	cd nj-derive; RUST_LOG=debug cargo test derive_ui -- --nocapture

test-try:
	cd nj-derive; RUST_LOG=debug cargo test derive_try -- --nocapture


#
#  Various Lint tools
#

install-fmt:
	rustup component add rustfmt --toolchain $(RUSTV)

check-fmt:
	cargo +$(RUSTV) fmt -- --check

install-clippy:
	rustup component add clippy --toolchain $(RUSTV)

check-clippy:	install-clippy
	cargo +$(RUSTV) clippy --all-targets  -- -D warnings