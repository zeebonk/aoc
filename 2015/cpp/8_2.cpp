#include <iostream>
#include <string>
#include <algorithm>
using namespace std;

int characters_of_code(const string &line) {
    return accumulate(
        line.begin(),
        line.end(),
        2,
        [](int sum, char c){ return sum + 1 + (c == '"' || c == '\\'); }
    );
}

int main() {
    auto total = 0;

    string line;
    while (getline(cin, line)) {
        total += (characters_of_code(line) - line.length());
    }

    cout << total << endl;
}
