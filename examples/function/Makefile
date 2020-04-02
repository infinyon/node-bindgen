
DYLIB = ../target/debug/libnj_example_function.dylib

all:	build


build:	
	nj-cli build || find .. | sort

test:	build
	node test.js


clean:
	rm -rf dylib