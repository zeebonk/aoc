import re
from functools import partial


OP_REGEXES = [
	('NOT (\S+)',          lambda wires, circuit: ~circuit[wires[0]] +  2**16),
	('(\S+) RSHIFT (\S+)', lambda wires, circuit:  circuit[wires[0]] >> circuit[wires[1]]),
	('(\S+) LSHIFT (\S+)', lambda wires, circuit:  circuit[wires[0]] << circuit[wires[1]]),
	('(\S+) OR (\S+)',     lambda wires, circuit:  circuit[wires[0]] |  circuit[wires[1]]),
	('(\S+) AND (\S+)',    lambda wires, circuit:  circuit[wires[0]] &  circuit[wires[1]]),
	('(\S+)',              lambda wires, circuit:  circuit[wires[0]]),
]


class Circuit(object):
	def __init__(self):
		self.gates_by_output_wire = {}
		self.cache = {}

	def __getitem__(self, wire):
		if wire.isdigit():
			self.cache[wire] = int(wire)
		elif wire not in self.cache:
			self.cache[wire] = self.gates_by_output_wire[wire](self)
		return self.cache[wire]

	def __setitem__(self, wire, value):
		self.gates_by_output_wire[wire] = value


circuit = Circuit()
for line in open('input/day_7.txt'):
	line = line.strip()
	for pattern, gate in OP_REGEXES:
		result = re.match('^' + pattern + ' -> (\S+)$', line)
		if result:
			groups = list(result.groups())
			target_wire = groups[-1]
			source_wires = groups[:-1]
			circuit[target_wire] = partial(gate, source_wires)
			break
	else:
		raise Exception("Unable to parse line")

print circuit['a']
