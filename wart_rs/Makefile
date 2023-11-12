.PHONY: package
package:
	wasm-pack build --release --target web
	cd pkg && npm pkg set type='module' && npm pkg set main='wart_rs.js'

.PHONY: clean
clean:
	git clean -dfx
