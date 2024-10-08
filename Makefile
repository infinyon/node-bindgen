RUSTV=stable

install_windows_on_mac:
	rustup target add x86_64-pc-windows-gnu
	brew install mingw-w64


build-windows:
	cargo build --target=x86_64-pc-windows-gnu


test-all:	test-derive test-examples

test-unit:
	cargo test --lib --all-features

test-examples:
	make -C examples test

test-derive:
	cd nj-derive; RUST_LOG=debug cargo test -- --nocapture


#
#  Various Lint tools
#

install-fmt:
	rustup component add rustfmt --toolchain $(RUSTV)

check-fmt:
	cargo fmt -- --check

install-clippy:
	rustup component add clippy

check-clippy:	install-clippy check-clippy-examples
	cargo clippy --all --all-features -- \
		-D warnings \
		-A clippy::upper_case_acronyms \
		-A clippy::needless-question-mark \
		-A clippy::macro-metavars-in-unsafe

check-clippy-examples: install-clippy
	make -C examples check-clippy
