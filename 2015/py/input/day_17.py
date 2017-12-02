from itertools import combinations

data = map(int, open('input/day_17.txt'))

print '17.1', sum(1 for i in xrange(len(data)) for x in combinations(data, i + 1) if sum(x) == 150)

min_boxes = min(i + 1 for i in xrange(len(data)) for x in combinations(data, i + 1) if sum(x) == 150)
print '17.2', sum(1 for x in combinations(data, min_boxes) if sum(x) == 150)
