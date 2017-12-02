#include <string>
#include <map>

using signal = uint16_t;

enum class OpType { ALIAS, NOT, AND, OR, LSHIFT, RSHIFT, VALUE };

struct Operation {
	OpType op_type;
	std::string in1;
	std::string in2;
	std::string out;
	signal value;
};

signal solve(Operation &op, std::map<std::string, Operation> &operations);
