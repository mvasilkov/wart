'use strict'

const fs = require('node:fs/promises')

async function run() {
    const { default: init, lexer } = await import('wart_rs')

    await init(fs.readFile(require.resolve('wart_rs/wart_rs_bg.wasm')))

    lexer('(module (func))', (kind, offset, len, text) => {
        if (kind instanceof Error) {
            console.error(kind)
            return
        }
        // offset and len are in bytes
        console.log('%o %o %o %o', kind, offset, len, text)
    })
}

if (require.main === module) {
    run()
}
