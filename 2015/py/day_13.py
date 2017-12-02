import re
from itertools import permutations, product
from collections import defaultdict

weights = defaultdict(int)
names = set()
for line in open('input/day_13.txt'):
	name_a, mode, amount, name_b = re.match(
		'(\S+) would (gain|lose) (\d+) happiness units by sitting next to (\S+)\.',
		line
	).groups()
	weights[(name_a, name_b)] += int(amount) if mode == 'gain' else -int(amount)
	weights[(name_b, name_a)] += int(amount) if mode == 'gain' else -int(amount)
	names |= {name_a, name_b}

print max(sum(weights[pair] for pair in zip(setting, setting[1:] + (setting[0], ))) for setting in product(names, repeat=len(names)))
