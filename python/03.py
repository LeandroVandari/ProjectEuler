
factors = []

largest_factor = 0
prev_prime = 2
file = "/Users/administrador/Desktop/Python/ProjectEuler/primes.txt"
with open(file, 'r') as f:
    primes = f.readlines()
print(primes)
if len(primes) > 1:
    a = int(primes[-1]) + 2
else:
    with open(file, "w") as f:
        f.write(str(2))
    a = 3
print(a)
while a < 1200000:
    if a % 997 == 0:
        print(a)
    is_prime = True
    for prime in primes:
        if is_prime == True:
            if (a / prev_prime) > int(prime):
                if a % int(prime) == 0:
                    is_prime = False
            prev_prime = int(prime)
    if is_prime == True:
        with open(file, "r") as f:
            primes.append(a)
            b = f.readlines()
            last_line = int(b[-1])
        with open(file, "a") as f:
            if a > last_line:
                f.write("\n")
                f.write(str(a))

    a += 2
f.close()
    
print(primes)
print(len(primes))
not_done = True
number = 600851475143
while not_done:
    factor = 0
    for prime in primes:
        if (number % int(prime)) == 0:
            if int(prime) > factor:
                factor = int(prime)
                if int(prime) > largest_factor:
                    largest_factor = int(prime)
    number = number / factor
    if number == 1:
        not_done = False
print(largest_factor)
        
        
