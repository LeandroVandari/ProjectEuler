a = 2
not_done = True
final_number = 0
while not_done:
    b = 2
    keep_going = True
    while keep_going:
        if b <= 20:
            if not (a / b).is_integer():
                break
            
        else:
            final_number = a
            keep_going = False
            not_done = False
            break
        b += 1
    a += 2 
print(final_number)
