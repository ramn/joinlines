SHELL=bash

test:
	cargo build && \
		diff <(./target/debug/joinlines 5 < ./resources/testfile.txt) \
			./resources/expected.txt
