clean:
	cargo clean
	make -C function clean
	make -C async-cb clean
	make -C cb clean
	make -C promise clean
	make -C stream clean
	make -C json clean
	make -C class-simple clean
	make -C class-wrapper clean
	make -C class-async clean
	make -C param clean


test: test-function test-cb test-async-cb test-promise test-json test-class-simple \
	test-class-wrapper test-class-async test-stream test-buffer test-array

test-function:
	make -C function test

test-async-cb:
	make -C async-cb test

test-cb:
	make -C cb test

test-promise:
	make -C promise test

test-json:
	make -C json test

test-stream:
	make -C stream test

test-class-simple:
	make -C class-simple test

test-class-wrapper:
	make -C class-wrapper test

test-class-async:
	make -C class-async test

test-param:
	make -C param test

test-buffer:
	make -C buffer test

test-array:
	make -C array test	