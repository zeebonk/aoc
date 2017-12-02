#include <iostream>
#include <array>
#include <algorithm>
using namespace std;

int main() {
    int length = 0;
    int width = 0;
    int height = 0;
    int sum = 0;
    char x = ' ';

    while (cin >> length >> x >> width >> x >> height) {
        array<int, 3> sides{
            length * width,
            width * height,
            height * length
        };
        // Total surface
        sum += 2 * sides[0] + 2 * sides[1] + 2 * sides[2];
        // Slack
        sum += *min_element(begin(sides), end(sides));
    }

    cout << sum << endl;
}
