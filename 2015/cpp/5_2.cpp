#include <iostream>
using namespace std;

/*
A nice string is one with all of the following properties:

  - It contains a pair of any two letters that appears at least twice in the
    string without overlapping, like xyxy (xy) or aabcdefgaa (aa), but not like
    aaa (aa, but it overlaps).
  - It contains at least one letter which repeats with exactly one letter
    between them, like xyx, abcdefeghi (efe), or even aaa.
*/

bool is_nice(const string &line) {
    if (line.size() < 3) {
        return false;
    }

    auto has_pair_with_gap = false;
    auto has_double_pair = false;

    auto c1 = '\0';
    auto c2 = line[0];
    auto c3 = line[1];

    for (decltype(line.size()) i = 1; i < line.size(); ) {
        has_pair_with_gap |= (c1 == c3);
        has_double_pair |= line.find(string{c2, c3}, i + 1) != line.npos;

        c1 = c2;
        c2 = c3;
        c3 = line[++i];
    }

    return has_double_pair && has_pair_with_gap;
}

int main() {
    auto n_nice = 0u;

    string line;
    while (getline(std::cin, line)) {
        n_nice += is_nice(line) ? 1 : 0;
    }

    cout << n_nice << endl;
}
