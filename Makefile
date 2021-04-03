build:
	rm -rf build
	mkdir build
	sh -c 'rustc --out-dir build/ true/true.rs'
	sh -c 'rustc --out-dir build/ false/false.rs'
	sh -c 'rustc --out-dir build/ yes/yes.rs'
	sh -c 'rustc --out-dir build/ printenv/printenv.rs'


.PHONY: build
