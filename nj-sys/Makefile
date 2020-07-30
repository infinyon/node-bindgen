
install_bindgen:
	cargo install bindgen --version 0.54.1

generate:	install_bindgen
	bindgen --verbose node.h  -o src/binding.rs


clean:
	rm -rf src/binding.rs