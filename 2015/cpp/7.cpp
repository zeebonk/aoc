#include <map>
#include <iostream>
#include <string>
#include <regex>
#include "7.hpp"
using namespace std;

static const map<string, OpType> string_to_optype{
    {"NOT", OpType::NOT},
    {"AND", OpType::AND},
    {"OR", OpType::OR},
    {"LSHIFT", OpType::LSHIFT},
    {"RSHIFT", OpType::RSHIFT},
};

bool is_number(const std::string &s) {
    auto it = s.cbegin();
    while (it != s.cend() && std::isdigit(*it)) ++it;
    return !s.empty() && it == s.cend();
}

signal get_signal(const string &wire, map<string, Operation> &operations) {
    if (wire == "") {
        return 0;
    }
    else if (is_number(wire)) {
        return (signal)stoi(wire);
    }
    else {
        return solve(operations.at(wire), operations);
    }
}

signal solve(Operation &op, map<string, Operation> &operations) {
    if (op.op_type == OpType::VALUE) {
        return op.value;
    }

    signal in1 = get_signal(op.in1, operations);
    signal in2 = get_signal(op.in2, operations);

    switch (op.op_type) {
        case OpType::VALUE: break;
        case OpType::ALIAS:  op.value = in1;        break;
        case OpType::NOT:    op.value = ~in1;       break;
        case OpType::AND:    op.value = in1 & in2;  break;
        case OpType::OR:     op.value = in1 | in2;  break;
        case OpType::LSHIFT: op.value = in1 << in2; break;
        case OpType::RSHIFT: op.value = in1 >> in2; break;
    }

    op.op_type = OpType::VALUE;
    return op.value;
}

int main() {
    map<string, Operation> operations;

    regex binary_op_re("([^ ]+) (AND|OR|LSHIFT|RSHIFT) ([^ ]+) -> ([^ ]+)");
    regex unary_op_re("(NOT) ([^ ]+) -> ([^ ]+)");
    regex assignemnt_re("([^ ]+) -> ([^ ]+)");

    string line;
    smatch match;
    while (getline(cin, line)) {
        Operation op;
        if (regex_match(line, match, assignemnt_re)) {
            op.op_type = OpType::ALIAS;
            op.in1 = match[1];
            op.out = match[2];
        }
        else if (regex_match(line, match, unary_op_re)) {
            op.op_type = string_to_optype.at(match[1]);
            op.in1 = match[2];
            op.out = match[3];
        }
        else if (regex_match(line, match, binary_op_re)) {
            op.op_type = string_to_optype.at(match[2]);
            op.in1 = match[1];
            op.in2 = match[3];
            op.out = match[4];
        }
        else {
            cout << "Unsupported operation:" << endl << line << endl;
            break;
        }

        operations[op.out] = op;
    }

    cout << solve(operations.at("a"), operations) << endl;
}
