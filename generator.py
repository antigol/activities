from random import shuffle, randint
from copy import copy

fname = input('filename : ')

n = int(input('amount of people   : '))
m = int(input('amount of workshop : '))

nt = int(input('amount of typical choices : '))
np = int(input('amount of permutations    : '))

# create typical choices
at = list(range(m))
ts = []
for i in range(nt):
	shuffle(at)
	ts.append(copy(at))

# create VALUES
values = []
for i in range(n):
	x = copy(ts[randint(0, nt-1)])
	for j in range(np):
		a = randint(0, m-1)
		b = randint(0, m-1)
		x[a],x[b] = x[b],x[a]
	values.append(x)

# create VMIN VMAX
while True:
	vmin = []
	vmax = []
	nat = round(n / m)
	for i in range(m):
		a = randint(nat - nat // 2, nat + nat // 2)
		b = randint(a, nat + nat // 2)
		vmin.append(a)
		vmax.append(b)
	if sum(vmin) < n and sum(vmax) > n:
		break

# output into file
import csv
import sys

with open(fname, 'w', newline='') as f:
	wr = csv.writer(f, dialect='excel')
	wr.writerow(vmin)
	wr.writerow(vmax)
	wr.writerows(values)
