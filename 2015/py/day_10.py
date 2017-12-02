from itertools import chain

def consecutives_count(data):
	count = 0
	cur_c = data[0]
	for c in data:
		if c == cur_c:
			count += 1
		else:
			yield [count, cur_c]
			cur_c = c
			count = 1
	yield [count, cur_c]

input = map(int, '1113122113')
for i in xrange(50):
	input = list(chain.from_iterable(consecutives_count(input)))
print len(''.join(map(str, input)))
