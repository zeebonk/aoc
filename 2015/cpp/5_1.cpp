#include <iostream>
using namespace std;

/*
A string is nice if:

  - It containts at least three vowels (aeiou only), like aei, xazegov, or
    aeiouaeiouaeiou.
  - It contains at least one letter that appears twice in a row, like xx,
    abcdde (dd), or aabbccdd (aa, bb, cc, or dd).
  - It does not contain the strings ab, cd, pq, or xy, even if they are part of
    one of the other requirements.
*/

bool is_nice(const string &line) {
    if (line.size() < 2) {
        return false;
    }

    auto has_pairs = false;
    auto n_vowels = 0u;

    auto c1 = '\0';
    auto c2 = line[0];
    for (decltype(line.size()) i = 0; i < line.size(); ) {
        switch (c1) {
            case 'a':
            case 'c':
            case 'p':
            case 'x':
                if (c1 + 1 == c2) return false;
        }

        switch (c2) {
            case 'a':
            case 'e':
            case 'i':
            case 'o':
            case 'u':
                ++n_vowels;
        }

        has_pairs |= (c1 == c2);

        c1 = c2;
        c2 = line[++i];
    }

    return has_pairs && n_vowels >= 3;
}

int main() {
    auto n_nice = 0u;

    string line;
    while (getline(std::cin, line)) {
        n_nice += is_nice(line) ? 1 : 0;
    }

    cout << n_nice << endl;
}
