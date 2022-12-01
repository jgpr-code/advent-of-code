#include <iostream>
using namespace std;
using ll = long long;

static const ll MAGIC = 20201227;
static const ll INITIAL_SUBJECT = 7;

// card loop_size
// door loop_size

// inputs:
// card -> subject 7 -> card pub key
// door -> subject 7 -> door pub key

ll determine_loop_size(ll subject_number, ll pub_key) {
  // start with 1
  // loop_size times:
  // - value *= subject_number
  // - value %= MAGIC
  ll loop_size = 0;
  ll value = 1;
  while (value != pub_key) {
    value *= subject_number;
    value %= MAGIC;
    ++loop_size;
  }
  return loop_size;
}

ll get_encryption_key(ll subject_number, ll loop_size) {
  ll value = 1;
  while (loop_size-- > 0) {
    value *= subject_number;
    value %= MAGIC;
  }
  return value;
}

int main() {
  ll card_pub_key, door_pub_key;
  cin >> card_pub_key >> door_pub_key;
  cout << card_pub_key << endl;
  cout << door_pub_key << endl;

  ll test_pub_key = 5764801;
  cout << "test: " << determine_loop_size(INITIAL_SUBJECT, test_pub_key)
       << endl;

  ll card_loop_size = determine_loop_size(INITIAL_SUBJECT, card_pub_key);
  cout << "Part 1: " << get_encryption_key(door_pub_key, card_loop_size)
       << endl;
}