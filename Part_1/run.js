const fs = require('fs');
const wasmBuffer = fs.readFileSync('./recursive.wasm');
WebAssembly.instantiate(wasmBuffer).then(result => {
    const { RecursiveCount } = result.instance.exports;
    for (let i = 0; i < 10; i++) {
        console.log(`RecursiveCount(${i}) = ${RecursiveCount(i)}`);
    }
});