#include <algorithm>
#include <iostream>
#include <regex>
#include <vector>
using namespace std;
int findSeat(int lower, int upper, char takeLower, const string& instructions) {
  for (size_t i = 0; i < instructions.size(); ++i) {
    int beginOfUpperHalf = (lower + upper + 1) / 2;
    if (instructions[i] == takeLower) {
      upper = beginOfUpperHalf - 1;
    } else {
      lower = beginOfUpperHalf;
    }
  }
  return lower;
}
int getSeatId(int seatRow, int seatCol) { return seatRow * 8 + seatCol; }
int findFirstMissingNumberInSorted(const vector<int>& numbers) {
  for (size_t i = 1; i < numbers.size(); ++i) {
    if (numbers[i - 1] + 1 != numbers[i]) return numbers[i - 1] + 1;
  }
  return -1;  // TODO throw an exception instead
}
int main() {
  regex rg("[FB]{7}[LR]{3}");
  int highestSeatId = numeric_limits<int>::min();
  vector<int> allSeatIds;
  for (string line; getline(cin, line);) {
    if (!regex_match(line, rg)) continue;
    int seatRow = findSeat(0, 127, 'F', line.substr(0, 7));
    int seatCol = findSeat(0, 7, 'L', line.substr(7, 3));
    int seatId = getSeatId(seatRow, seatCol);
    highestSeatId = max(highestSeatId, seatId);
    allSeatIds.push_back(seatId);
  }
  cout << "Part 1: " << highestSeatId << endl;
  sort(allSeatIds.begin(), allSeatIds.end());
  int mySeatId = findFirstMissingNumberInSorted(allSeatIds);
  cout << "Part 2: " << mySeatId << endl;
}
