a = 0
b = 0
largest_palindrome = 0
while a != 999:
    b = 0
    while b != 999:
        c = str(b * a)
        if c[::-1] == c:
            if int(c) > largest_palindrome:
                largest_palindrome = int(c)
                largest_factor_1 = a
                largest_factor_2 = b
        b += 1
    a += 1
    if (a % 100) == 0:
        print(a)

print(largest_palindrome)
print(largest_factor_1, largest_factor_2)
