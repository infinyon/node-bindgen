all:	build

build:
	nj-cli build -- -p nj-example-uuid

test:	build
	npx -y ts-node test.ts

clean:
	rm -rf dist

