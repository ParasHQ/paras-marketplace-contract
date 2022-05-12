release:
	$(call docker_build,_rust_setup.sh)
	mkdir -p release
	cp target/wasm32-unknown-unknown/release/paras_marketplace_contract.wasm release/paras_marketplace_contract.wasm
	cp release/paras_marketplace_contract.wasm out/main.wasm

define docker_build
	docker build -t my-contract-builder .
	docker run \
		--mount type=bind,source=${PWD},target=/host \
		--cap-add=SYS_PTRACE --security-opt seccomp=unconfined \
		-w /host \
		-e RUSTFLAGS=$(RFLAGS) \
		-i -t my-contract-builder \
		/bin/bash $(1)
endef
