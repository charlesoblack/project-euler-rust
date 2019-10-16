#!/usr/bin/env python3

from timeit import default_timer as timer
from math import sqrt

def is_prime(x):
    for num in range(2, int(sqrt(x)) + 1):
        if x % num == 0:
            return False
    return True

start = timer()
goal_num = 600851475143
curr = 2
largest_prime = curr

while curr < sqrt(goal_num):
    curr += 1;
    if goal_num % curr == 0 and is_prime(curr):
        largest_prime = curr

print(largest_prime)
print('Took {:.10f} seconds.'.format(timer() - start))
