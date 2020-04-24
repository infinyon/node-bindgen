clean:
	make -C function clean
	make -C async-cb clean
	make -C cb clean
	make -C promise clean
	make -C class clean
	make -C stream clean

test: test-function test-cb test-class test-promise test-stream

test-function:
	make -C function test

test-async-cb:
	make -C async-cb test

test-cb:
	make -C cb test

test-promise:
	make -C promise test

test-class:
	make -C class test

test-stream:
	make -C stream test	