all: build

install:
	npm install

build: install
	nj-cli build

test: build
	npx ts-node test.ts

clean:
	rm -rf dist

