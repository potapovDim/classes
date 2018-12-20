const ffi = require('ffi')

const lib = ffi.Library('./target/release/libclass_3.dylib', {
  'process': ['void', []]
})

lib.process()

console.log('done')