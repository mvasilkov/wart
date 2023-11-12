import { readFile } from 'node:fs/promises'
import init, { lexer } from './pkg/wart_rs.js'

await init(readFile(new URL('./pkg/wart_rs_bg.wasm', import.meta.url)))

lexer('(module (func))', (kind, offset, len, text) => {
    // offset and len are in bytes
    if (kind instanceof Error) {
        console.error(kind)
        return
    }
    console.log('%o %o %o %o', kind, offset, len, text)
})
