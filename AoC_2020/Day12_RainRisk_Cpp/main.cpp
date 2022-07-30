#include <iostream>
#include <regex>
#include <string>
#include <tuple>

using namespace std;

class Coordinate2d {
 private:
  int x;
  int y;

 public:
  static const Coordinate2d North;
  static const Coordinate2d East;
  static const Coordinate2d South;
  static const Coordinate2d West;

  static Coordinate2d directionFromLetterOrDefault(
      char directionLetter, const Coordinate2d &defaultDirection) {
    switch (directionLetter) {
      case 'N':
        return North;
      case 'E':
        return East;
      case 'S':
        return South;
      case 'W':
        return West;
      default:
        return defaultDirection;
    }
  }

  Coordinate2d(int x_coordinate, int y_coordinate)
      : x(x_coordinate), y(y_coordinate) {}

  Coordinate2d &operator+=(const Coordinate2d &rhs) {
    x += rhs.x;
    y += rhs.y;
    return *this;
  }

  friend Coordinate2d operator+(Coordinate2d, const Coordinate2d &);

  Coordinate2d &operator-=(const Coordinate2d &rhs) {
    x -= rhs.x;
    y -= rhs.y;
    return *this;
  }

  friend Coordinate2d operator-(Coordinate2d, const Coordinate2d &);

  Coordinate2d &operator*=(int rhs) {
    x *= rhs;
    y *= rhs;
    return *this;
  }

  friend Coordinate2d operator*(Coordinate2d, const Coordinate2d &);

  void rotateRight(int deg90) {
    switch (deg90 % 4) {
      case 1:
        swap(x, y);
        y = -y;
        break;
      case 2:
        x = -x;
        y = -y;
        break;
      case 3:
        swap(x, y);
        x = -x;
        break;
    }
  }

  void rotateLeft(int deg90) {
    switch (deg90 % 4) {
      case 1:
        swap(x, y);
        x = -x;
        break;
      case 2:
        x = -x;
        y = -y;
        break;
      case 3:
        swap(x, y);
        y = -y;
    }
  }

  int getX() const { return x; }

  int getY() const { return y; }

  int manhattan() const { return abs(x) + abs(y); }
};

const Coordinate2d Coordinate2d::North(0, 1);
const Coordinate2d Coordinate2d::East(1, 0);
const Coordinate2d Coordinate2d::South(0, -1);
const Coordinate2d Coordinate2d::West(-1, 0);

inline Coordinate2d operator+(Coordinate2d lhs, const Coordinate2d &rhs) {
  lhs += rhs;
  return lhs;
}

inline Coordinate2d operator-(Coordinate2d lhs, const Coordinate2d &rhs) {
  lhs -= rhs;
  return lhs;
}

inline Coordinate2d operator*(Coordinate2d lhs, int rhs) {
  lhs *= rhs;
  return lhs;
}

inline Coordinate2d operator*(int lhs, Coordinate2d rhs) {
  rhs *= lhs;
  return rhs;
}

class Ship {
  Coordinate2d position;
  Coordinate2d direction;
  Coordinate2d waypoint;

  tuple<char, int> parseCourse(const string &command) {
    regex re("(N|E|S|W|L|R|F)([[:digit:]]+)");
    smatch matches;
    regex_match(command, matches, re);
    char dirLetter = matches[1].str()[0];
    int amount = stoi(matches[2].str());
    return make_tuple(dirLetter, amount);
  }

 public:
  Ship() : position(0, 0), direction(Coordinate2d::East), waypoint(10, 1) {}

  void takeDirectionalCourse(const string &course) {
    char dirLetter;
    int amount;
    tie(dirLetter, amount) = parseCourse(course);
    switch (dirLetter) {
      case 'L':
        direction.rotateLeft(amount / 90);
        break;
      case 'R':
        direction.rotateRight(amount / 90);
        break;
      default:
        position +=
            Coordinate2d::directionFromLetterOrDefault(dirLetter, direction) *
            amount;
        break;
    }
  }

  void takeWaypointCourse(const string &course) {
    char dirLetter;
    int amount;
    tie(dirLetter, amount) = parseCourse(course);
    switch (dirLetter) {
      case 'L':
        waypoint.rotateLeft(amount / 90);
        break;
      case 'R':
        waypoint.rotateRight(amount / 90);
        break;
      case 'F':
        position += waypoint * amount;
        break;
      default:
        waypoint +=
            Coordinate2d::directionFromLetterOrDefault(dirLetter, direction) *
            amount;
        break;
    }
  }

  int getManhattan() const { return position.manhattan(); }
};

int main() {
  Ship ship1;
  Ship ship2;
  for (string line; getline(cin, line);) {
    ship1.takeDirectionalCourse(line);
    ship2.takeWaypointCourse(line);
  }
  cout << "Part 1: " << ship1.getManhattan() << endl;  // 1424
  cout << "Part 2: " << ship2.getManhattan() << endl;
}