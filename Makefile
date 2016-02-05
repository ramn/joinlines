SHELL=bash

test: test_with_default_separator test_with_separator
	@echo "All tests pass"

build:
	cargo build

test_with_default_separator:
	@diff <(./target/debug/joinlines 5 < ./resources/testfile.txt) \
		./resources/expected.txt

test_with_separator:
	@diff <(./target/debug/joinlines 5 ' --!-- ' < ./resources/testfile.txt) \
		./resources/expected2.txt
