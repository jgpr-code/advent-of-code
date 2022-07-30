#include <iostream>
#include <string>
#include <vector>

using namespace std;

class HorizontallyRepeatingForest {
 private:
  size_t myBaseLineLength;
  vector<string> myBaseLines;

  bool isOutOfBounds(int x, int y) {
    return x < 0 || y < 0 || y >= myBaseLines.size();
  }

  bool isInBounds(int x, int y) { return !isOutOfBounds(x, y); }

  bool hasTree(int x, int y) {
    if (isOutOfBounds(x, y)) return false;
    int baseX = x % myBaseLineLength;
    return myBaseLines[y][baseX] == '#';
  }

 public:
  HorizontallyRepeatingForest(size_t baseLineLength, vector<string> baseLines)
      : myBaseLines(move(baseLines)), myBaseLineLength(baseLineLength) {}

  int countTreesOnSlope(int slopeX, int slopeY) {
    return countTreesOnSlope(0, 0, slopeX, slopeY);
  }
  int countTreesOnSlope(int startX, int startY, int slopeX, int slopeY) {
    int count = 0;
    int posX = startX;
    int posY = startY;
    while (isInBounds(posX, posY)) {
      if (hasTree(posX, posY)) ++count;
      posX += slopeX;
      posY += slopeY;
    }
    return count;
  }
};

int main() {
  vector<string> baseLines;
  size_t baseLineLength = 0;
  for (string line; getline(cin, line);) {
    baseLines.push_back(line);
    if (baseLineLength == 0) baseLineLength = line.size();
  }
  HorizontallyRepeatingForest forest(baseLineLength, baseLines);
  cout << "Part 1: " << forest.countTreesOnSlope(3, 1) << endl;

  long long slope11 = forest.countTreesOnSlope(1, 1);
  long long slope31 = forest.countTreesOnSlope(3, 1);
  long long slope51 = forest.countTreesOnSlope(5, 1);
  long long slope71 = forest.countTreesOnSlope(7, 1);
  long long slope12 = forest.countTreesOnSlope(1, 2);

  cout << "Part 2: " << slope11 * slope31 * slope51 * slope71 * slope12 << endl;
}