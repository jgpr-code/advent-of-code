#include <algorithm>
#include <iostream>
#include <unordered_map>
#include <vector>

using namespace std;
using ull = unsigned long long;

class XMAS_Validator {
  vector<ull> allNumbers;
  vector<ull> cumulativeSums;
  size_t firstActiveIdx;
  size_t lastActiveIdx;
  unordered_map<ull, int> possibleSums;

  void shiftActiveRight() {
    ull removeNumber = allNumbers[firstActiveIdx];
    ull addNumber = allNumbers[lastActiveIdx + 1];
    for (int i = firstActiveIdx + 1; i <= lastActiveIdx; ++i) {
      ull ni = allNumbers[i];
      if (removeNumber != ni) {
        ull removeSum = removeNumber + ni;
        possibleSums[removeSum] -= 1;
      }
      if (addNumber != ni) {
        ull addSum = addNumber + ni;
        if (possibleSums.count(addSum) == 0) possibleSums.emplace(addSum, 0);
        possibleSums[addSum] += 1;
      }
    }
    ++firstActiveIdx;
    ++lastActiveIdx;
  }

  bool isValid(ull number) {
    return possibleSums.count(number) && possibleSums[number] > 0;
  }

  ull querySum(size_t from, size_t to) {
    return allNumbers[from] + cumulativeSums[to] - cumulativeSums[from];
  }

 public:
  XMAS_Validator(vector<ull> numbers, vector<ull> sums)
      : allNumbers(move(numbers)),
        cumulativeSums(move(sums)),
        firstActiveIdx(0),
        lastActiveIdx(24) {
    for (size_t i = firstActiveIdx; i < lastActiveIdx; ++i) {
      ull ni = allNumbers[i];
      for (size_t j = i + 1; j <= lastActiveIdx; ++j) {
        ull nj = allNumbers[j];
        if (ni == nj) continue;
        ull sum = ni + nj;
        if (possibleSums.count(sum) == 0) possibleSums.emplace(sum, 0);
        possibleSums[sum] += 1;
      }
    }
  }

  ull findFirstInvalid() {
    while (lastActiveIdx + 1 < allNumbers.size()) {
      ull nextNumber = allNumbers[lastActiveIdx + 1];
      if (!isValid(nextNumber)) return nextNumber;
      shiftActiveRight();
    }
    return 0;
  }

  ull findWeakness(ull key) {
    for (size_t i = 0; i < allNumbers.size(); ++i) {
      for (size_t j = i; j < allNumbers.size(); ++j) {
        if (querySum(i, j) == key) {
          const auto minmax = minmax_element(allNumbers.begin() + i,
                                             allNumbers.begin() + j + 1);
          return *minmax.first + *minmax.second;
        }
      }
    }
    return 0;
  }
};

int main() {
  ull in;
  vector<ull> numbers;
  ull sum = 0;
  vector<ull> sums;
  while (cin >> in) {
    numbers.push_back(in);
    sum += in;
    sums.push_back(sum);
  }
  cout << "max elem: " << *max_element(numbers.begin(), numbers.end())
       << endl;  // safe to sum in ull!
  XMAS_Validator xmas(move(numbers), move(sums));
  ull key = xmas.findFirstInvalid();
  cout << "Part 1: " << key << endl;
  cout << "Part 2: " << xmas.findWeakness(key) << endl;
}