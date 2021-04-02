build:
	rm -rf build
	mkdir build
	sh -c 'rustc --out-dir build/ true/true.rs'
	sh -c 'rustc --out-dir build/ false/false.rs'


.PHONY: build
