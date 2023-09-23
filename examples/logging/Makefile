all:	build

build:
	nj-cli build

test:	build
	RUST_LOG=info node test.js

clean:
	rm -rf dist
