#include <iostream>
#include <set>
using namespace std;

int main() {
    pair<int, int> position;
    set<decltype(position)> visited_positions;

    char token = ' ';
    while (cin >> token) {
        switch (token) {
            case '>': position.first += 1; break;
            case '<': position.first -= 1; break;
            case '^': position.second += 1; break;
            case 'v': position.second -= 1; break;
            default: break;
        }

        visited_positions.insert(position);
    }

    cout << visited_positions.size() << endl;
}
