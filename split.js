const fs = require('fs')
const data = fs.readFileSync('./cows.txt', 'utf8')
const split = data.split('\n\n\n')
const nCows = split.length
const code = `pub static COWS: [&str; ${nCows}] = [
`

const cowsCode = split.reduce((prev, current, index) => {
    const cow = current
        .replace(/\\/gi, '\\\\')
        .replace(/"/gi, '\\"')

    return prev + '"' + cow + (index < nCows - 1 ? '",\n' : '"')
}, '')

const file = code + cowsCode + '\n];'

fs.writeFileSync('./src/cows.rs', file, 'utf8')