#!/usr/bin/env python3

from timeit import default_timer as timer

start = timer()

my_sum = 0
num_1 = 1
num_0 = 1

while num_0 < 4e6:
    if num_0 % 2 == 0:
        my_sum += num_0
    num_0, num_1 = num_0 + num_1, num_0

print(my_sum)
print('Took {:.10f} seconds.'.format(timer() - start))
