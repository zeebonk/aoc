from itertools import permutations

weights = {}
cities = set()
for line in open('input/day_9.txt'):
	parts = line[:-1].split(' ')
	cities |= {parts[0], parts[2]}
	pair = tuple(sorted((parts[0], parts[2])))
	weights[pair] = int(parts[4])

total_dist = 0
for perm in permutations(cities):
	dist = 0
	for pair in zip(perm, perm[1:]):
		pair = tuple(sorted(pair))
		dist += weights[pair]
	total_dist = max(total_dist, dist)

print total_dist
