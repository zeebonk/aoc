#include <iostream>
#include <string>
using namespace std;

int characters_in_memory(const string &line) {
    int total = 0;

    string::size_type i = 1;
    while (i < (line.length() - 1)) {
        ++total;

        // Non-escape character
        if (line[i] != '\\') {
            ++i;
            continue;
        }

        auto remaining = line.length() - 1 - i;
        /* Escape: \xXX */
        if (remaining >= 4 && line[i + 1] == 'x' && isxdigit(line[i + 2]) && isxdigit(line[i + 3])) {
            i += 4;
        }
        /* Escape: \" and \\ */
        else if (remaining >= 2 && (line[i + 1] == '"' || line[i + 1] == '\\')) {
            i += 2;
        }
        /* No escape */
        else {
            i += 1;
        }
    }

    return total;
}

int main() {
    auto total = 0;

    string line;
    while (getline(cin, line)) {
        total += (line.length() - characters_in_memory(line));
    }

    cout << total << endl;
}
