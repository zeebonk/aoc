#include <iostream>
using namespace std;

int main() {
    int floor = 0;
    char token = ' ';

    while (cin >> token) {
        floor += token == '(' ? 1 : -1;
    }

    cout << floor << endl;
}
