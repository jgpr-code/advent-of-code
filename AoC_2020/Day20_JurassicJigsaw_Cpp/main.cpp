#include <iostream>
#include <map>
#include <regex>
#include <vector>

using namespace std;
using ll = long long;

class Tile {
public:
  enum State {
    ORIGINAL = 0,
    ROTATED_90 = 1,
    ROTATED_180 = 2,
    ROTATED_270 = 3,
    FLIPPED_270 = 4,
    FLIPPED_ORIGINAL = 5,
    FLIPPED_90 = 6,
    FLIPPED_180 = 7
  };

  // [row][col]
  vector<vector<char>> myContent;
  State myState;
  ll myId;

  Tile() : myState(State::ORIGINAL), myId(-1) {}

  size_t rows() const {
    return myContent.size();
  }

  size_t cols() const {
    if (myContent.size() == 0) {
      return 0;
    }
    return myContent[0].size();
  }

  static State next_state(State state) {
    return static_cast<State>((state + 1) % 8);
  }

  void rotate_ccw_90() {
    //   0 1 2
    //   3 4 5
    //   6 7 8

    //   2 5 8
    //   1 4 7
    //   0 3 6
    vector<vector<char>> old_content = myContent;
    for (int row = 0; row < rows(); ++row) {
      for (int col = 0; col < cols(); ++col) {
        size_t old_row = col;
        size_t old_col = cols() - 1 - row;
        myContent[row][col] = old_content[old_row][old_col];
      }
    }
  }

  void flip_horizontal() {
    // 1 2 3
    // 4 5 6
    // 7 8 9

    // 7 8 9
    // 4 5 6
    // 1 2 3
    vector<vector<char>> old_content = myContent;
    for (int row = 0; row < rows(); ++row) {
      for (int col = 0; col < cols(); ++col) {
        size_t old_row = rows() - 1 - row;
        size_t old_col = col;
        myContent[row][col] = old_content[old_row][old_col];
      }
    }
  }

  void next_orientation() {
    switch (myState) {
    case State::ROTATED_270:
    case State::FLIPPED_180:
      flip_horizontal();
      break;
    default:
      rotate_ccw_90();
      break;
    }
    myState = next_state(myState);
  }

  bool matches_tile_toLeft(const Tile& toLeft) {
    if (rows() != toLeft.rows() || cols() != toLeft.cols()) {
      return false;
    }
    for (size_t row = 0; row < rows(); ++row) {
      if (toLeft.myContent[row][toLeft.cols() - 1] != myContent[row][0]) {
        return false;
      }
    }
    return true;
  }

  bool matches_tile_toTop(const Tile& toTop) {
    if (rows() != toTop.rows() || cols() != toTop.cols()) {
      return false;
    }
    for (size_t col = 0; col < cols(); ++col) {
      if (toTop.myContent[toTop.rows() - 1][col] != myContent[0][col]) {
        return false;
      }
    }
    return true;
  }

  ll count_hashes() const {
    ll count = 0;
    for (size_t row = 0; row < rows(); ++row) {
      for (size_t col = 0; col < cols(); ++col) {
        if (myContent[row][col] == '#') { ++count; }
      }
    }
    return count;
  }

  void crop_borders() {
    auto old_content = myContent;
    myContent = vector<vector<char>>(old_content.size() - 2, vector<char>(old_content[0].size() - 2, ' '));
    for (size_t row = 0; row < myContent.size(); ++row) {
      for (size_t col = 0; col < myContent[0].size(); ++col) {
        myContent[row][col] = old_content[row + 1][col + 1];
      }
    }
  }

  void mark_seamonsters() {
    /* A seamonster :)
X                 #
#    ##    ##    ###
 #  #  #  #  #  #
    */
    // (drow, dcol)
    vector<pair<size_t, size_t>> monster_hash_offsets =
    { {0, 18},
      {1, 0}, {1, 5}, {1, 6}, {1, 11}, {1, 12}, {1, 17}, {1, 18}, {1, 19},
      {2, 1}, {2, 4}, {2, 7}, {2, 10}, {2, 13}, {2, 16} };
    for (int orient = 0; orient < 8; ++orient) {
      for (size_t row = 0; row < rows(); ++row) {
        for (size_t col = 0; col < cols(); ++col) {
          if (row + 2 < rows() && col + 19 < cols()) {
            bool is_monster = true;
            for (const auto offset : monster_hash_offsets) {
              if (myContent[row + offset.first][col + offset.second] != '#') {
                is_monster = false;
                break;
              }
            }
            if (is_monster) {
              for (const auto offset : monster_hash_offsets) {
                myContent[row + offset.first][col + offset.second] = 'O';
              }
            }
          }
        }
      }
      next_orientation();
    }

  }

  friend istream& operator>>(istream& is, Tile& tile)
  {
    static int call = 0;
    ++call;
    cout << "called istream >> " << call << endl;
    string line;
    getline(is, line);
    regex re("Tile (\\d+):");
    smatch matches;
    if (!regex_match(line, matches, re))
    {
      cerr << "wrong tile header!" << endl;
    }
    tile.myId = stoll(matches[1].str());
    while (getline(is, line))
    {
      if (line == "") {
        break;
      }
      cout << line << endl;
      tile.myContent.push_back(vector<char>());
      for (char c : line) {
        tile.myContent.back().push_back(c);
      }
    }
    return is;
  }

  friend ostream& operator<<(ostream& os, const Tile& tile)
  {
    os << "Id " << tile.myId << endl;
    os << "Lines" << endl;
    for (const auto& line : tile.myContent)
    {
      for (char c : line) {
        os << c;
      }
      os << endl;
    }
    return os;
  }
};

class TileImage {
public:
  using used_t = bool;
  vector<pair<used_t, Tile>> myTiles;
  // only stores the index to myTiles
  vector<vector<int>> myImage;
  size_t myRows;
  size_t myCols;

  TileImage(size_t rows, size_t cols) : myRows(rows), myCols(cols) {
    myImage = vector<vector<int>>(rows + 1, vector<int>(cols + 1, -1));
  }
  void ReadNextTile() {
    myTiles.push_back(make_pair(false, Tile()));
    cin >> myTiles.back().second;
  }
  Tile get_tile(size_t row, size_t col) {
    return myTiles[myImage[row][col]].second;
  }
  bool solve(size_t row, size_t col) {
    //cout << "Solve " << row << ", " << col << endl;
    //cout << *this;

    if (row > myRows) {
      return true;
    }
    for (size_t tile_idx = 0; tile_idx < myTiles.size(); ++tile_idx) {
      auto& tile = myTiles[tile_idx];
      if (tile.first) {
        continue;
      }
      for (int orient = 0; orient < 8; ++orient) {
        if ((myImage[row - 1][col] == -1 || tile.second.matches_tile_toTop(myTiles[myImage[row - 1][col]].second)) &&
          (myImage[row][col - 1] == -1 || tile.second.matches_tile_toLeft(myTiles[myImage[row][col - 1]].second)))
        {
          myImage[row][col] = tile_idx;
          tile.first = true;
          size_t new_row = row;
          size_t new_col = col + 1;
          if (new_col > myCols) {
            new_row = row + 1;
            new_col = 1;
          }
          if (solve(new_row, new_col)) {
            return true;
          }
          tile.first = false;
          myImage[row][col] = -1;
        }
        tile.second.next_orientation();
      }
    }
    return false;
  }

  Tile assemble_as_tile() {

    for (auto& tile : myTiles) {
      tile.second.crop_borders();
    }
    Tile image;
    size_t tile_size = myTiles.front().second.rows();
    size_t image_size = tile_size * myRows;
    image.myContent = vector<vector<char>>(image_size, vector<char>(image_size, ' '));
    for (size_t row = 1; row <= myRows; ++row) {
      size_t row_offset = (row - 1) * tile_size;
      for (size_t col = 1; col <= myCols; ++col) {
        size_t col_offset = (col - 1) * tile_size;
        const auto& tile = myTiles[myImage[row][col]].second;
        for (size_t tile_row = 0; tile_row < tile.rows(); ++tile_row) {
          for (size_t tile_col = 0; tile_col < tile.cols(); ++tile_col) {
            image.myContent[tile_row + row_offset][tile_col + col_offset] = tile.myContent[tile_row][tile_col];
          }
        }
      }
    }

    return image;
  }

  friend ostream& operator<<(ostream& os, const TileImage& image) {
    for (size_t row = 0; row <= image.myRows; ++row) {
      for (size_t col = 0; col <= image.myRows; ++col) {
        os << image.myImage[row][col] << " ";
      }
      os << endl;
    }
    os << endl;
    return os;
  }
};

bool Tile_uTests();
bool TileImage_uTests();

int main()
{
  const int IMAGE_SIZE = 12;
  if (!Tile_uTests() || !TileImage_uTests()) {
    cout << "uTests were not successful, aborting...";
    return 1;
  }

  TileImage image(IMAGE_SIZE, IMAGE_SIZE);
  int read_tiles = 0;
  while (true)
  {
    image.ReadNextTile();
    ++read_tiles;
    if (!cin)
      break;
  }
  cout << "Read " << read_tiles << " tiles" << endl;
  ll factor = 1;
  if (image.solve(1, 1)) {
    factor *= image.get_tile(1, 1).myId;
    factor *= image.get_tile(1, IMAGE_SIZE).myId;
    factor *= image.get_tile(IMAGE_SIZE, 1).myId;
    factor *= image.get_tile(IMAGE_SIZE, IMAGE_SIZE).myId;
  } else {
    cout << "Could not solve :(" << endl;
  }

  // sea monster stuff
  Tile monster_image = image.assemble_as_tile();

  cout << monster_image << endl;
  monster_image.mark_seamonsters();
  cout << monster_image << endl;

  cout << "Part 1: " << factor << endl;
  cout << "Part 2: " << monster_image.count_hashes() << endl;
}

// ***************************** UTEST ********************************************

vector<vector<char>> CreateDefaultContent() {
  return
  { {'#','.','#'},
    {'.','.','#'},
    {'.','#','.'} };
}

Tile CreateDefaultTile() {
  Tile defaultTile;
  defaultTile.myContent = CreateDefaultContent();
  return defaultTile;
}

bool Tile_uTests() {

  bool error = false;
  // flip twice -> identity
  Tile flipTwice = CreateDefaultTile();
  flipTwice.flip_horizontal();
  flipTwice.flip_horizontal();
  auto expected = CreateDefaultContent();
  if (flipTwice.myContent != expected) {
    cerr << "2x flipping the tile didn't work" << endl;
    return false;
  }

  // rotate ccw -> rotated
  Tile rotate = CreateDefaultTile();
  rotate.rotate_ccw_90();
  expected =
  { {'#','#','.'},
    {'.','.','#'},
    {'#','.','.'} };
  if (rotate.myContent != expected) {
    cerr << "rotate didn't work" << endl;
    for (int row = 0; row < 3; ++row) {
      for (int col = 0; col < 3; ++col) {
        cerr << rotate.myContent[row][col] << " ";
      }
      cerr << endl;
    }
    error = true;
  }
  // next_orientation 16times -> identity
  Tile nextOrient = CreateDefaultTile();
  for (int i = 0; i < 16; ++i) { nextOrient.next_orientation(); }
  expected = CreateDefaultContent();
  if (nextOrient.myContent != expected) {
    cerr << "next_orient didn't work" << endl;
    error = true;
  }

  // matches top
  Tile matchTop = CreateDefaultTile();
  Tile matchingTop = CreateDefaultTile();
  matchingTop.flip_horizontal();
  if (!matchTop.matches_tile_toTop(matchingTop)) {
    cerr << "matchingTop didn't work" << endl;
    error = true;
  }

  // doesn't match top
  Tile notmatchTop = CreateDefaultTile();
  Tile notmatchingTop = CreateDefaultTile();
  if (notmatchTop.matches_tile_toTop(notmatchingTop)) {
    cerr << "matchingTop didn't work (no match)" << endl;
    error = true;
  }

  // matches left
  Tile matchLeft = CreateDefaultTile();
  Tile matchingLeft = CreateDefaultTile();
  matchingLeft.rotate_ccw_90();
  matchingLeft.rotate_ccw_90();
  matchingLeft.flip_horizontal();
  if (!matchLeft.matches_tile_toLeft(matchingLeft)) {
    cerr << "matchingLeft didn't work" << endl;
    error = true;
  }

  // doesn't match left
  Tile notmatchLeft = CreateDefaultTile();
  Tile notmatchingLeft = CreateDefaultTile();
  if (notmatchLeft.matches_tile_toLeft(notmatchingLeft)) {
    cerr << "matchingLeft didn't work (not match)" << endl;
    error = true;
  }

  // crop borders
  Tile cropBorders = CreateDefaultTile();
  cropBorders.crop_borders();
  if (cropBorders.myContent != vector<vector<char>>({ {'.'} })) {
    cerr << "crop borders didn't work" << endl;
    error = true;
  }

  return !error;
}

bool TileImage_uTests() {
  return true;
}
