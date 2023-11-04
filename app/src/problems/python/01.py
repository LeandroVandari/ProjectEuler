a = 1
numero_final = 0
while a != 1000:
    if (a / 3).is_integer() or (a / 5).is_integer():
        numero_final += a
    a +=1
print(numero_final)
