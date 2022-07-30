#include <iostream>
#include <regex>
#include <string>

using namespace std;

struct password_policy {
  char c;
  int min_c;
  int max_c;
};

using password_t = string;

pair<password_policy, password_t> parse_line(const string& line) {
  regex r("^([0-9]*)-([0-9]*) ([[:alpha:]]): ([[:alpha:]]*)$", regex::egrep);
  smatch match_results;
  bool success = regex_match(line, match_results, r);
  if (!success) {
    cout << "something went wrong with: " << line << endl;
  }

  password_policy p;
  p.c = (match_results[3].str())[0];
  p.min_c = stoi(match_results[1].str());
  p.max_c = stoi(match_results[2].str());
  return make_pair(p, match_results[4].str());
}

int count_c(char c, const string& str) {
  int count = 0;
  for (char x : str) {
    if (x == c) ++count;
  }
  return count;
}

bool is_valid_password_part_one(const password_policy& policy,
                                const string& password) {
  int count = count_c(policy.c, password);
  return policy.min_c <= count && count <= policy.max_c;
}

bool is_valid_password_part_two(const password_policy& policy,
                                const string& password) {
  int a = policy.min_c - 1;  // needs to be zero indexed
  int b = policy.max_c - 1;  // needs to be zero indexed
  return (password[a] == policy.c) ^ (password[b] == policy.c);
}

int main() {
  vector<password_policy> policies;
  vector<string> passwords;
  for (string line; getline(cin, line);) {
    auto info = parse_line(line);
    policies.push_back(info.first);
    passwords.push_back(info.second);
  }

  int correct_part_one = 0;
  int correct_part_two = 0;
  for (size_t i = 0; i < passwords.size(); ++i) {
    if (is_valid_password_part_one(policies[i], passwords[i]))
      ++correct_part_one;
    if (is_valid_password_part_two(policies[i], passwords[i]))
      ++correct_part_two;
  }
  cout << "Part 1: " << correct_part_one << endl;
  cout << "Part 2: " << correct_part_two << endl;
}