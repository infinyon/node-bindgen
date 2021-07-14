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
	make -C electron clean
	make -C bigint clean
	make -C tuple clean
	make -C cleanup clean


test: test-function test-cb test-async-cb test-promise test-json test-class-simple \
	test-class-wrapper test-class-async test-stream test-buffer test-array test-bigint \
	test-cleanup

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

test-electron:
	make -C electron test

test-bigint:
	make -C bigint test

test-tuple:
	make -C tuple test

test-cleanup:
	make -C cleanup test

check-clippy:
	cargo +$(RUSTV) clippy --all --all-targets --all-features -- \
		-D warnings \
		-A clippy::upper_case_acronyms \
		-A clippy::needless-question-mark
