#include <algorithm>
#include <iostream>
#include <vector>

using namespace std;
using ll = long long;

bool is_inside(int val, int low, int high) { return val >= low && val <= high; }

bool can_connect(ll low_volatage, ll high_voltage) {
  ll voltage_difference = high_voltage - low_volatage;
  return voltage_difference >= 1 && voltage_difference <= 3;
}

int main() {
  ll in;
  vector<ll> adapter_volatages;
  while (cin >> in) {
    adapter_volatages.push_back(in);
  }
  adapter_volatages.push_back(0);
  sort(adapter_volatages.begin(), adapter_volatages.end());
  adapter_volatages.push_back(adapter_volatages.back() + 3);

  ll n_voltage_difference_1 = 0;
  ll n_voltage_difference_3 = 0;

  for (size_t i = 0; i < adapter_volatages.size() - 1; ++i) {
    ll voltage_difference = adapter_volatages[i + 1] - adapter_volatages[i];
    if (voltage_difference == 1) ++n_voltage_difference_1;
    if (voltage_difference == 3) ++n_voltage_difference_3;
  }

  cout << "Part 1: " << n_voltage_difference_1 * n_voltage_difference_3 << endl;

  vector<ll> possibilities(adapter_volatages.size(), 0);
  possibilities.back() = 1;
  for (int i = possibilities.size() - 1; i >= 0; --i) {
    ll possibilities_here = possibilities[i];
    for (int j = 1; j <= 3; ++j) {
      int pred = i - j;
      if (is_inside(pred, 0, possibilities.size() - 1) &&
          can_connect(adapter_volatages[pred], adapter_volatages[i])) {
        possibilities[i - j] += possibilities_here;
      }
    }
  }

  cout << "Part 2: " << possibilities[0] << endl;
}