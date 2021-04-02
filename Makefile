build:
	rm -rf build
	mkdir build
	sh -c 'rustc --out-dir build/ true/true.rs'


.PHONY: build
