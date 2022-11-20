package main

import (
	"fmt"
	"time"
)

func isPrime(num int) bool {
	if num <= 1 {
		return false
	}
	for i := 2; i <= num/2; i++ {
		if num%i == 0 {
			return false
		}
	}
	return true
}

func findPrime(num int) int {
	var largestPrime = 0
	for j := 2; j <= num; j++ {
		if isPrime(j) {
			largestPrime = j
		}
	}
	return largestPrime
}

func main() {
	var before = time.Now().UnixNano()/int64(time.Millisecond);
	fmt.Println("prime: ", findPrime(500000))
	fmt.Println("elapsed: ", (time.Now().UnixNano()/int64(time.Millisecond)- before)/1000)
}
