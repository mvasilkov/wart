'use strict'

const fs = require('node:fs/promises')
const { program } = require('commander')
const { version } = require('./package.json')

const createLexer = async () => {
    const { default: init, lexer } = await import('wart_rs')

    const wasm = require.resolve('wart_rs/wart_rs_bg.wasm')
    await init(fs.readFile(wasm))

    return lexer
}

let lexer = null

const fmt = async wasm => {
    if (!lexer) {
        [lexer, wasm] = await Promise.all([createLexer(), wasm])
    }
    else wasm = await wasm

    const result = []

    lexer(wasm, (kind, text) => {
        if (kind instanceof Error) throw kind

        result.push(text)
    }, false)

    return result.join('')
}

const cli = async files => {
    for (const file of files) {
        const wasm = fs.readFile(file, 'utf8')
        const result = await fmt(wasm)

        console.log(result)
    }
}

const run = async () => {
    program
        .version(version)
        .argument('<files...>')
        .action(cli)
        .parse()
}

if (require.main === module) run()
else module.exports = fmt
