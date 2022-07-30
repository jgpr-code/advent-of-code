#include <iostream>
#include <string>
#include <vector>

using namespace std;

enum class PersonBehavior { NORMAL, NITPICKING };

class SeatMap {
  static const char occupied_seat = '#';
  static const char empty_seat = 'L';
  static const char floor = '.';

  vector<string> seats;
  int sx;
  int sy;
  const vector<int> dx;
  const vector<int> dy;

  bool is_index_valid(int y, int x) {
    return x >= 0 && y >= 0 && x < sx && y < sy;
  }

  char get_udpdated_seat(int y, int x, PersonBehavior person_behavior) {
    bool update_occured = false;
    int count_surrounding_occupied = 0;
    for (size_t i = 0; i < dy.size(); ++i) {
      for (int j = 1;; ++j) {
        int ny = y + j * dy[i];
        int nx = x + j * dx[i];
        if (!is_index_valid(ny, nx)) break;
        if (seats[ny][nx] == occupied_seat) {
          ++count_surrounding_occupied;
          break;
        }
        if (seats[ny][nx] == empty_seat) break;
        if (person_behavior == PersonBehavior::NORMAL) break;
      }
    }
    int max_tolerated = (person_behavior == PersonBehavior::NORMAL) ? 4 : 5;
    if (seats[y][x] == occupied_seat &&
        count_surrounding_occupied >= max_tolerated) {
      return empty_seat;
    }
    if (seats[y][x] == empty_seat && count_surrounding_occupied == 0) {
      return occupied_seat;
    }
    return seats[y][x];
  }

  bool step(PersonBehavior person_behavior) {
    vector<string> next_seats = seats;
    bool has_changed = false;
    for (int y = 0; y < sy; ++y) {
      for (int x = 0; x < sx; ++x) {
        next_seats[y][x] = get_udpdated_seat(y, x, person_behavior);
        has_changed |= (seats[y][x] != next_seats[y][x]);
      }
    }
    if (has_changed) seats = move(next_seats);
    return has_changed;
  }

 public:
  SeatMap()
      : sx(-1),
        sy(-1),
        dx({0, 1, 1, 1, 0, -1, -1, -1}),
        dy({-1, -1, 0, 1, 1, 1, 0, -1}) {}

  void initialize_from_stream(istream& stream) {
    for (string line; getline(stream, line);) {
      seats.push_back(line);
    }
    sy = static_cast<int>(seats.size());
    if (sy > 0) sx = static_cast<int>(seats[0].size());
  }

  void set_to_fixpoint(PersonBehavior person_behavior) {
    while (step(person_behavior)) {
    }
  }

  int count_occupied() const {
    int count = 0;
    for (int y = 0; y < sy; ++y) {
      for (int x = 0; x < sx; ++x) {
        if (seats[y][x] == occupied_seat) ++count;
      }
    }
    return count;
  }
};

int main() {
  SeatMap seat_map;
  seat_map.initialize_from_stream(cin);
  SeatMap seat_map_copy = seat_map;

  seat_map.set_to_fixpoint(PersonBehavior::NORMAL);
  cout << "Part 1: " << seat_map.count_occupied() << endl;  // 2354
  seat_map_copy.set_to_fixpoint(PersonBehavior::NITPICKING);
  cout << "Part 2: " << seat_map_copy.count_occupied() << endl;
}