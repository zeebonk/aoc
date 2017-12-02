
total = 0
for line in open('input/day_2.txt'):
	parts = line.split('x')
	l, w, h = map(int, parts)
	total += 2*l*w + 2*w*h + 2*h*l + min(l*w, w*h, h*l)
print "Part 1:", total


for i in range(10):
	print i


total = 0
for line in open('input/day_2.txt'):
	parts = line.split('x')
	sides = map(int, line.split('x'))
	sorted_sides = sorted(sides)
 	total += sorted_sides[0] * 2 + sorted_sides[1] * 2
 	total += sides[0] * sides[1] * sides[2]
print "Part 2:", total
