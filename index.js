const findPrime = require("./find_primes")
const findPrimeNative = require("./build/Release/find_primes")

const input = 500000

let result

console.time("cpp")
result = findPrimeNative(input)
console.timeEnd("cpp")
console.log(result)

console.time("JS")
result = findPrime(input)
console.timeEnd("JS")
console.log(result)