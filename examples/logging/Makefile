all:	build

build:
	nj-cli build -- -p nj-example-logging

test:	build
	RUST_LOG=info node test.js

clean:
	rm -rf dist
