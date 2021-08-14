TARGET_FLAG=$(if $(TARGET),--target $(TARGET),)

install_rustup_target:
	./build-scripts/install_target.sh

install_windows_on_mac:
	rustup target add x86_64-pc-windows-gnu
	brew install mingw-w64


build-windows:
	cargo build --target=x86_64-pc-windows-gnu


test-all:	test-unit test-derive test-examples

test-unit:	install_rustup_target 
	cargo test --lib --all-features $(TARGET_FLAG)

test-examples:
	make -C examples test


buid-examples:
	make -C examples build

test-derive:
	cd nj-derive; RUST_LOG=debug cargo test $(TARGET_FLAG) -- --nocapture


#
#  Various Lint tools
#

install-fmt:
	rustup component add rustfmt

check-fmt:
	cargo fmt -- --check

install-clippy:
	rustup component add clippy

check-clippy:	install-clippy check-clippy-examples
	cargo clippy --all --all-features -- \
		-D warnings \
		-A clippy::upper_case_acronyms \
		-A clippy::needless-question-mark

check-clippy-examples: install-clippy
	make -C examples check-clippy
