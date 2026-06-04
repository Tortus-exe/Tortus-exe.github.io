publish: 
	@rm -rf docs/
	RUSTFLAGS='--cfg getrandom_backend="wasm_js"' dx bundle --out-dir docs
	@mv -f docs/public/* docs
	@cp docs/index.html docs/404.html