'use strict'

const { readFile } = require('node:fs/promises')

import('../pkg/wart_rs.js').then(async ({ default: init, lexer }) => {
    await init(readFile(require.resolve('../pkg/wart_rs_bg.wasm')))

    lexer('(module (func))', (kind, offset, len, text) => {
        if (kind instanceof Error)
            return console.error(kind)

        // offset and len are in bytes
        console.log('%o %o %o %o', kind, offset, len, text)
    })
})
