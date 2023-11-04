def sum_of_squares(lst: [int]) -> int:
    sum = 0
    for number in lst:
        sum += number**2
    return sum
def square_of_sums(lst: [int]) -> int:
    sum = 0
    for number in lst:
        sum += number
    return sum**2

answer = square_of_sums(list(range(101))) - sum_of_squares(list(range(101)))
print(answer)
