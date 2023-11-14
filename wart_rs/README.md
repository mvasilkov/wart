# wart_rs

WebAssembly Text Format (WAT) lexer from the [wast][wast] crate, compiled to WebAssembly for use in JavaScript.

[wast]: https://crates.io/crates/wast

## Frontend usage

Run in the browser:

```js
import init, { lexer } from './node_modules/wart_rs/wart_rs.js'

await init()

lexer('(module (func))', (kind, offset, len, text) => {
    if (kind instanceof Error)
        return console.error(kind)
    // offset and len are in bytes
    console.log('%o %o %o %o', kind, offset, len, text)
})
```

## Backend usage

Works in [Bun][bun]!

```js
import { readFile } from 'node:fs/promises'
import init, { lexer } from 'wart_rs/wart_rs.js'

const wasm = new URL(import.meta.resolve('wart_rs/wart_rs_bg.wasm'))
await init(readFile(wasm))

lexer('(module (func))', (kind, offset, len, text) => {
    if (kind instanceof Error)
        return console.error(kind)
    // offset and len are in bytes
    console.log('%o %o %o %o', kind, offset, len, text)
})
```

[bun]: https://bun.sh/

## Backend usage (CommonJS modules)

Since 2009.

```js
const { readFile } = require('node:fs/promises')

import('wart_rs/wart_rs.js').then(async ({ default: init, lexer }) => {
    const wasm = require.resolve('wart_rs/wart_rs_bg.wasm')
    await init(readFile(wasm))

    lexer('(module (func))', (kind, offset, len, text) => {
        if (kind instanceof Error)
            return console.error(kind)
        // offset and len are in bytes
        console.log('%o %o %o %o', kind, offset, len, text)
    })
})
```

## Tokens

The lexer yields tokens of the following types:

- **LineComment**
- **BlockComment** (can be nested)
- **Whitespace**
- **LParen**
- **RParen**
- **String**
- **Id** (start with `$`)
- **Keyword**
- **Reserved** (unexpected token)
- **Integer**
- **Float**
