#include <cassert>
#include <deque>
#include <iostream>
#include <map>
#include <set>
#include <sstream>
#include <string>
#include <vector>

using namespace std;

class ConwayCubes {
 private:
  static const char active;
  static const char inactive;
  using CubeIndex = vector<int>;
  int dims;
  set<CubeIndex> active_cubes;

  void dfsNeighbors(vector<CubeIndex>& neighbors, int pos, CubeIndex idx) {
    if (pos == idx.size()) {
      neighbors.push_back(idx);
      return;
    }
    for (int d = -1; d <= 1; ++d) {
      idx[pos] += d;
      dfsNeighbors(neighbors, pos + 1, idx);
      idx[pos] -= d;
    }
  }

  vector<CubeIndex> neighbors(const CubeIndex& idx) {
    vector<CubeIndex> ret;
    dfsNeighbors(ret, 0, idx);
    return ret;
  }

  void step() {
    map<CubeIndex, int> countActiveNeighbors;
    for (CubeIndex idx : active_cubes) {
      for (CubeIndex neighbor : neighbors(idx)) {
        countActiveNeighbors.try_emplace(neighbor, 0);
        countActiveNeighbors[neighbor]++;
      }
      countActiveNeighbors[idx]--;  // don't count yourself in your own field
    }
    for (const auto kvp : countActiveNeighbors) {
      CubeIndex idx = kvp.first;
      int n = kvp.second;
      if (n == 3 || (n == 2 && active_cubes.count(idx))) {
        active_cubes.insert(idx);
      } else {
        active_cubes.erase(idx);
      }
    }
  }

 public:
  ConwayCubes(int dimensions, int rows, int cols, string init)
      : dims(dimensions) {
    assert(init.length() == rows * cols);
    int idx = 0;
    for (int row = 0; row < rows; ++row) {
      for (int col = 0; col < cols; ++col) {
        if (init[idx] == active) {
          vector<int> cube_idx = {row, col};
          cube_idx.resize(dimensions);
          active_cubes.insert(cube_idx);
        }
        ++idx;
      }
    }
  }

  void simulate(int steps) {
    while (steps > 0) {
      --steps;
      step();
    }
  }

  int countActive() const { return active_cubes.size(); }
};
const char ConwayCubes::active = '#';
const char ConwayCubes::inactive = '.';

int main() {
  string init;
  int rows = 0;
  int cols = 0;
  for (string line; getline(cin, line);) {
    init.append(line);
    ++rows;
    cols = line.length();
  }

  cout << init << endl;

  ConwayCubes cubes3d(3, rows, cols, init);
  cubes3d.simulate(6);
  cout << "Part 1: " << cubes3d.countActive() << endl;
  ConwayCubes cubes4d(4, rows, cols, init);
  cubes4d.simulate(6);
  cout << "Part 2: " << cubes4d.countActive() << endl;
}
