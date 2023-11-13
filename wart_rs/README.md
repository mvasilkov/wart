# wart_rs

WebAssembly Text Format (WAT) lexer from the [wast][wast] crate, compiled to WebAssembly for use in JavaScript.

[wast]: https://crates.io/crates/wast

## Frontend usage

Run in the browser:

```js
import init, { lexer } from './node_modules/wart_rs/wart_rs.js'

await init()

lexer('(module (func))', (kind, offset, len, text) => {
    if (kind instanceof Error) {
        console.error(kind)
        return
    }
    // offset and len are in bytes
    console.log('%o %o %o %o', kind, offset, len, text)
})
```

## Backend usage

Works in [Bun][bun]!

```js
import { readFile } from 'node:fs/promises'
import init, { lexer } from './node_modules/wart_rs/wart_rs.js'

await init(readFile(new URL('./node_modules/wart_rs/wart_rs_bg.wasm', import.meta.url)))

lexer('(module (func))', (kind, offset, len, text) => {
    if (kind instanceof Error) {
        console.error(kind)
        return
    }
    // offset and len are in bytes
    console.log('%o %o %o %o', kind, offset, len, text)
})
```

[bun]: https://bun.sh/
