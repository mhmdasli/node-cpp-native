import time

def isPrime(num :int):
  i: int
  max = int(num/2)
  if (num <= 1):
    return False
  for i in range(2,max):
    if num % i == 0:
      return False
  return True


def findPrime(num):
  largestPrime = 0
  for j in range(2,num): 
    if (isPrime(j)):
      largestPrime = j
  return largestPrime

now = time.time()
print("prime = " + float(findPrime(500000)).__str__())
print("elapsed: " + (time.time()- now).__str__())
