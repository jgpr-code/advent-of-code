#include <algorithm>
#include <iostream>
#include <string>
#include <vector>

using namespace std;

long long find_sum_in_sorted(int sum, int start_index,
                             const vector<int>& nums) {
  size_t f = start_index;
  size_t b = nums.size();
  while (f < b) {
    long long nf = nums[f];
    long long nb = nums[b];
    if (nf + nb == sum) {
      cout << "found: " << nf << " + " << nb << " = " << sum << endl;
      cout << "product: " << nf * nb << endl;
      return nf * nb;
    }
    if (nf + nb > sum) {
      --b;
    } else {
      ++f;
    }
  }
  cout << "no two numbers add up to " << sum << endl;
  return -1;
}

int main() {
  vector<int> nums;
  for (string line; getline(cin, line);) {
    nums.push_back(stoi(line));
  }
  std::sort(nums.begin(), nums.end());

  find_sum_in_sorted(2020, 0, nums);

  for (int i = 0; i < nums.size(); ++i) {
    long long res = nums[i] * find_sum_in_sorted(2020 - nums[i], i + 1, nums);
    if (res > 0) {
      cout << "i: " << i << endl;
      cout << "ni: " << nums[i] << endl;
      cout << "answer: " << res << endl;
      break;
    }
  }
}