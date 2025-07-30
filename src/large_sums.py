#!/usr/bin/env python3

with open("./resources/large_sum.txt") as f:
    sum = 0
    for line in f:
        bigInt = int(line.strip())
        sum += bigInt
        print(bigInt)
print(str(sum)[0:10])
