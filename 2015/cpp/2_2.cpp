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
        array<int, 3> half_perimeters{
            length + width,
            width + height,
            height + length,
        };
        // Smallest perimeter
        sum += 2 * *min_element(begin(half_perimeters), end(half_perimeters));
        // Bow
        sum += width * length * height;
    }

    cout << sum << endl;
}
