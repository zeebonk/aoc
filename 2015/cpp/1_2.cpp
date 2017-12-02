#include <iostream>
using namespace std;

int main() {
    int index = 0;
    int floor = 0;
    char token = ' ';

    while (cin >> token) {
        floor += token == '(' ? 1 : -1;
        index += 1;
        if (floor == -1) {
            break;
        }
    }

    cout << index << endl;
}
