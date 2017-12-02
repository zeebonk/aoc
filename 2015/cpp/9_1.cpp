#include <iostream>
#include <string>
#include <regex>
#include <vector>
#include <algorithm>
#include <unordered_map>
using namespace std;

using city = int;
using picker = int const & (*) (int const &, int const &);

int path_length(picker pick_best, int init, int n_cities, map<pair<city, city>, int> &weights) {
    int best_length = init;

    vector<city> ids(n_cities);
    iota(ids.begin(), ids.end(), 0);

    do {
        int current_length = 0;
        for (auto it = ids.begin(); it != ids.end() - 1; ++it) {
            auto p = make_pair(*it, *(it + 1));
            const auto pair = weights.find(p);
            if (pair == weights.end()) {
                break;
            }
            current_length += pair->second;
        }
        best_length = pick_best(best_length, current_length);
    } while (next_permutation(ids.begin(), ids.end()));

    return best_length;
}

int main() {
    map<string, city> name_to_id_map;
    map<pair<city, city>, int> weights_map;

    const regex re("([^ ]+) to ([^ ]+) = (\\d+)");
    smatch match;
    string line;
    while (getline(cin, line) && regex_match(line, match, re)) {
        if (name_to_id_map.find(match[1]) == name_to_id_map.end()) {
            name_to_id_map[match[1]] = (int)name_to_id_map.size();
        }

        if (name_to_id_map.find(match[2]) == name_to_id_map.end()) {
            name_to_id_map[match[2]] = name_to_id_map.size();
        }

        auto city_a = name_to_id_map[match[1]];
        auto city_b = name_to_id_map[match[2]];
        int weight = stoi(match[3]);
        weights_map[make_pair(city_a, city_b)] = weight;
        weights_map[make_pair(city_b, city_a)] = weight;
    }

    int n_cities = name_to_id_map.size();
    cout << "Min: " << path_length(min<int>, 10000, n_cities, weights_map) << endl;
    cout << "Max: " << path_length(max<int>, 0, n_cities, weights_map) << endl;
}