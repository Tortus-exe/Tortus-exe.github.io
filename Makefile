publish: 
	@rm -rf docs/
	dx bundle --out-dir docs
	@mv -f docs/public/* docs
	@cp docs/index.html docs/404.html