
install_bindgen:
	cargo install bindgen --version 0.52.0

generate:	install_bindgen
	bindgen --verbose node.h  -o src/binding.rs


clean:
	rm -rf src/binding.rs