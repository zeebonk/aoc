#include <array>
#include <iostream>
#include <map>
#include <regex>
#include <string>
using namespace std;

// Lamps definition
const int n_lamps = 1000;
using Lamps = array<array<bool, n_lamps>, n_lamps>;
using Ls = Lamps::size_type;

// Functions that work on the lamps
using Func = bool(*)(Lamps&, Ls, Ls);
bool turn_on(Lamps &lamps, Ls x, Ls y){ return lamps[y][x] = true; }
bool turn_off(Lamps &lamps, Ls x, Ls y){ return lamps[y][x] = false; }
bool toggle(Lamps &lamps, Ls x, Ls y) { return lamps[y][x] = !lamps[y][x]; }
bool count(Lamps &lamps, Ls x, Ls y){ return lamps[y][x]; }

// Applys `func` on `lamps` for the given range
int apply(Func func, Lamps &lamps, Ls c1x, Ls c1y, Ls c2x, Ls c2y) {
    int sum = 0;
    for (auto y = c1y; y < c2y; ++y) {
        for (auto x = c1x; x < c2x; ++x) {
            sum += func(lamps, x, y);
        }
    }
    return sum;
}

int main() {
    map<string, Func> modifiers{
       {"turn on", turn_on},
       {"turn off", turn_off},
       {"toggle", toggle},
    };

    Lamps lamps{{false}};

    string line;
    smatch match;
    regex re("(turn on|turn off|toggle) (\\d+),(\\d+) through (\\d+),(\\d+)");
    while (getline(cin, line) && regex_match(line, match, re)) {
        apply(
            modifiers[match[1]],
            lamps,
            stoi(match[2]),
            stoi(match[3]),
            stoi(match[4]) + 1,  // Given outer coordinates are inclusive
            stoi(match[5]) + 1
        );
    }

    cout << apply(count, lamps, 0, 0, n_lamps, n_lamps) << endl;
}
