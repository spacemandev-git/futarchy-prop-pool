.PHONY: build

build:
	anchor-0.29.0 build
	mkdir -p deps/
	cp target/idl/prop-pool.json deps/prop-pool.json
	cp target/types/prop-pool.ts deps/prop-pool.ts
	cp target/deploy/* deps/deploy/
