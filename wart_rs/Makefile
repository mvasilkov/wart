pkg_set := type='module' main='wart_rs.js' \
keywords[]='webassembly' keywords[]='wasm' keywords[]='wat' \
keywords[]='wast' keywords[]='lexer' keywords[]='parser'

.PHONY: package
package:
	wasm-pack build --release --target web
	cd pkg && npm pkg set $(pkg_set)

.PHONY: clean
clean:
	git clean -dfx
