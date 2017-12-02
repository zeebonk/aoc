import re
import json

print '12.1', sum(int(x.group(0)) for x in re.finditer('-*\d+', open('input/day_12.txt').read()))

traverse = lambda d: {int: lambda x: x, list: lambda x: sum(traverse(i) for i in x), dict: lambda x: 0 if 'red' in x.values() else sum(traverse(i) for i in x.values()), unicode: lambda x: 0,}[type(d)](d)
print '12.2', traverse(json.load(open('input/day_12.txt')))
