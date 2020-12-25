
install_bindgen:
	cargo install bindgen --version 0.56.0

generate:	install_bindgen
	bindgen --verbose node.h  -o src/binding.rs

# To compile on Ubuntu, may need to install 32bit lib
# sudo apt-get install gcc-multilib
generate_arm64:	install_bindgen
	bindgen --verbose node.h  --no-layout-tests -o src/binding_arm64.rs -- --target=aarch64-unknown-linux-gnu


clean:
	rm -rf src/binding.rs