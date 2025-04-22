.PHONY: build

build:
	anchor-0.29.0 build
	mkdir -p deps/ deps/deploy/
	cp target/idl/prop_pool.json deps/prop_pool.json
	cp target/types/prop_pool.ts deps/prop_pool.ts
	cp target/deploy/* deps/deploy/
