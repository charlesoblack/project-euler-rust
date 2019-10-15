#!/usr/bin/env python3

from timeit import default_timer as timer

start = timer()

my_sum = 0

for number in range(1000):
    if number % 3 == 0 or number % 5 == 0:
        my_sum += number

print(my_sum)
print('Took {:.10f} seconds.'.format(timer() - start))
