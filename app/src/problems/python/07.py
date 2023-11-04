import math
primes: [int] = [2]
num = 3
while len(primes) != 10001:
    primes_index: int = 0
    is_prime = True
    while primes[primes_index] <= math.sqrt(num):
        if num % primes[primes_index] == 0:
            is_prime = False
        primes_index +=1
    if is_prime:
        primes.append(num)
    num+=2
print(primes[-1])