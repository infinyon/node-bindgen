all: build

install:
	npm install

build: install
	nj-cli build -- -p nj-example-tuple

test: build
	npx ts-node test.ts

clean:
	rm -rf dist

