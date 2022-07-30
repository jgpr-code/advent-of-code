#include <iostream>
#include <sstream>
#include <unordered_map>
#include <vector>

using namespace std;
using ll = long long;

class Generator {
  ll idx;
  ll last;
  unordered_map<ll, pair<ll, ll>> history;

  void update_history(ll elem) {
    if (history.count(elem) == 0) {
      history.emplace(elem, make_pair(idx, -1));
    } else {
      auto old = history[elem];
      history[elem] = make_pair(idx, old.first);
    }
    last = elem;
    ++idx;
  }

  ll next_elem() const {
    auto history_p = history.at(last);
    if (history_p.second == -1) {
      return 0;
    }
    return history_p.first - history_p.second;
  }

  void next() { update_history(next_elem()); }

 public:
  Generator(const vector<ll>& seed) : idx(0), last(-1) {
    for (auto elem : seed) {
      update_history(elem);
    }
  }

  ll get_last() const { return last; }

  void generate_until(ll target_idx) {
    while (idx < target_idx) {
      next();
    }
  }
};

int main() {
  string line;
  getline(cin, line);
  cout << "line: " << line << endl;
  istringstream iss(line);
  vector<ll> seed;
  for (string num; getline(iss, num, ',');) {
    seed.push_back(stoi(num));
  }

  Generator gen(seed);
  gen.generate_until(2020);
  cout << "Part 1: " << gen.get_last() << endl;
  gen.generate_until(30000000);
  cout << "Part 2: " << gen.get_last() << endl;
}
