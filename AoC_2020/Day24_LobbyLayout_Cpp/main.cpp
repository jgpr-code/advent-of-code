#include <iostream>
#include <vector>
#include <regex>
#include <numeric>
#include <unordered_set>
#include <set>
#include <map>

using namespace std;

using location = pair<int, int>;

vector<string> tokenize(string input) {
    vector<string> tokens;
    regex hex_dirs_regex("(e|se|sw|w|nw|ne)");
    auto begin = sregex_iterator(input.begin(), input.end(), hex_dirs_regex);
    auto end = sregex_iterator();
    for (auto it = begin; it != end; ++it) {
        tokens.push_back(it->str());
    }
    return tokens;
}

/* How to model hexagonal coordinates

(row, col):

         (0,2)     (0,4)
    (1,1)     (1,3)     (1,5)
         (2,2)     (2,4)
*/
location get_delta(const string& dir) {
    if (dir == "e") {
        return make_pair(0, 2);
    }
    if (dir == "se") {
        return make_pair(1, 1);
    }
    if (dir == "sw") {
        return make_pair(1, -1);
    }
    if (dir == "w") {
        return make_pair(0, -2);
    }
    if (dir == "nw") {
        return make_pair(-1, -1);
    }
    if (dir == "ne") {
        return make_pair(-1, 1);
    }
    return make_pair(0, 0);
}

void show_tokenized(const vector<string>& tokens) {
    auto tokenized = accumulate(++tokens.begin(), tokens.end(), *tokens.begin(), [](const string& lhs, const string& rhs) {return lhs + "|" + rhs;});
    cout << tokenized << endl;
}

location get_target_tile(string path) {
    auto target_tile = make_pair(0, 0);
    auto tokens = tokenize(path);
    for (auto token : tokens) {
        auto delta = get_delta(token);
        target_tile.first += delta.first;
        target_tile.second += delta.second;
    }
    return target_tile;
}

set<location> simulate_day(const set<location>& black_tiles) {
    map<location, int> black_neighbours_count;
    vector<string> dirs = { "e", "se", "sw", "w", "nw", "ne" };
    for (auto loc : black_tiles) {
        // important to emplace the black_tile itself, because it might have no black neighbours
        black_neighbours_count.try_emplace(loc, 0);
        for (auto dir : dirs) {
            auto delta = get_delta(dir);
            auto neighbour_tile = make_pair(loc.first + delta.first, loc.second + delta.second);
            black_neighbours_count.try_emplace(neighbour_tile, 0);
            ++black_neighbours_count[neighbour_tile];
        }
    }
    set<location> new_black_tiles(black_tiles);
    for (auto kvp : black_neighbours_count) {
        auto tile = kvp.first;
        auto count = kvp.second;
        if (black_tiles.count(tile) == 1) {
            if (count == 0 || count > 2) {
                new_black_tiles.erase(tile);
            }
        } else if (count == 2) {
            new_black_tiles.insert(tile);
        }
    }
    return new_black_tiles;
}

void test() {
    auto center = get_target_tile("nwwswee");
    cout << "center: " << center.first << ", " << center.second << endl;
    string test = "sesenwnenenewseeswwswswwnenewsewsw";
    cout << test << endl;
    auto tokens = tokenize(test);
    show_tokenized(tokens);
}


int main() {
    //test();
    set<location> black_tiles;
    for (string line; getline(cin, line);) {
        auto target_tile = get_target_tile(line);
        if (black_tiles.count(target_tile) == 0) {
            black_tiles.insert(target_tile);
        } else {
            black_tiles.erase(target_tile);
        }
    }
    cout << "Part 1: " << black_tiles.size() << endl;
    for (int i = 1; i <= 100; ++i) {
        black_tiles = simulate_day(black_tiles);
        cout << "Day " << i << ": " << black_tiles.size() << endl;
    }

}