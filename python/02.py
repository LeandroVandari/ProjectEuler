numero_atual = 1
numero_anterior = 1
numero_final = 0
while numero_atual <= 4000000:
        numero_medio = numero_atual
        numero_atual += numero_anterior
        numero_anterior = numero_medio
        if (numero_atual / 2).is_integer():
                numero_final += numero_atual

print(numero_final)
