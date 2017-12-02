#include <iostream>
#include <set>
using namespace std;

int main() {
    pair<int, int> santa_pos, robot_pos;
    set<decltype(santa_pos)> visited_positions;
    bool alternator = true;

    char token = ' ';
    while (cin >> token) {
        auto &active_pos = alternator ? santa_pos : robot_pos;

        switch (token) {
            case '>': active_pos.first += 1; break;
            case '<': active_pos.first -= 1; break;
            case '^': active_pos.second += 1; break;
            case 'v': active_pos.second -= 1; break;
            default: break;
        }

        visited_positions.insert(active_pos);
        alternator = !alternator;
    }

    cout << visited_positions.size() << endl;
}
