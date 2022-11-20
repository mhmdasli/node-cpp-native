const findPrime = require("./find_primes")
const findPrimeNative = require("./build/Release/find_primes")

const input = 500000

let result

console.time("cpp elapsed")
result = findPrimeNative(input)
console.log("prime = " ,result)
console.timeEnd("cpp elapsed")

console.time("JS elapsed")
result = findPrime(input)
console.log("prime = " ,result)
console.timeEnd("JS elapsed")
