#include <iostream>
#include <regex>
#include <sstream>
#include <unordered_map>
#include <variant>
#include <vector>

using namespace std;

class Rule {
 private:
  using Part = variant<char, int>;
  using Target = vector<Part>;

  int id;
  vector<Target> targets;

 public:
  Rule(const string& rule_str) {
    // extract id
    size_t colon_pos = rule_str.find_first_of(':');
    id = stoi(rule_str.substr(0, colon_pos));

    // splitting the c++ way... (create stringstream and use getline)
    istringstream targets_stream(rule_str.substr(colon_pos + 1));
    for (string target_str; getline(targets_stream >> ws, target_str, '|');) {
      Target target;

      istringstream parts_stream(target_str);
      for (string part_str; getline(parts_stream >> ws, part_str, ' ');) {
        if (part_str.starts_with('\"') && part_str.ends_with('\"')) {
          target.push_back(Part(part_str[1]));
        } else {
          target.push_back(Part(stoi(part_str)));
        }
      }

      targets.push_back(target);
    }
  }

  int GetId() const { return id; }
  const vector<Target>& GetTargets() const { return targets; }

  friend ostream& operator<<(ostream& os, const Rule& rule) {
    os << "id = " << rule.id << ", targets: ";
    for (size_t i = 0; i < rule.targets.size(); ++i) {
      const auto& target = rule.targets[i];
      if (i != 0) os << " | ";
      for (size_t j = 0; j < target.size(); ++j) {
        const auto part = target[j];
        if (j != 0) os << " ";
        visit([&](auto&& arg) { os << arg; }, part);
      }
    }
    return os;
  }
};

class Ruleset {
 private:
  unordered_map<int, Rule> rules;

 public:
  string GetRegexForRule(int idx) const {
    const auto& rule = rules.at(idx);
    string regex_str;
    const auto& targets = rule.GetTargets();
    if (targets.size() > 1) regex_str.push_back('(');
    for (size_t i = 0; i < targets.size(); ++i) {
      const auto& target = targets[i];
      if (i != 0) regex_str.push_back('|');
      for (size_t j = 0; j < target.size(); ++j) {
        const auto part = target[j];
        if (part.index() == 0) {  // part is a char
          regex_str.push_back(get<0>(part));
        } else {  // part is a number
          regex_str.append(GetRegexForRule(get<1>(part)));
        }
      }
    }
    if (targets.size() > 1) regex_str.push_back(')');
    return regex_str;
  }

  void AddRule(const Rule rule) { rules.emplace(rule.GetId(), rule); }
  Rule GetRule(int idx) const { return rules.at(idx); }
};

namespace match_utils {
bool MatchesMoreAsThanBsWhenFullyMatched(const string& line, regex a, regex b) {
  int matched_as = 0;
  int matched_bs = 0;

  auto begin = line.begin();
  auto end = line.end();
  smatch matches;
  while (regex_search(begin, end, matches, a)) {
    if (begin != matches[0].first) break;
    begin = matches[0].second;
    ++matched_as;
  }
  while (regex_search(begin, end, matches, b)) {
    if (begin != matches[0].first) break;
    begin = matches[0].second;
    ++matched_bs;
  }
  return begin == end && matched_as > 1 && matched_bs > 0 &&
         matched_as > matched_bs;
}
}  // namespace match_utils

void Test() {
  cout << "******************** TEST ********************" << endl;
  string a = "abab";
  string b = "cddc";
  string c = "cdcd";
  string d = "argh";
  string e = "arghargh";

  string re_str = R"r((abab|cd(cd|dc)|argh){2,})r";
  regex re(re_str);
  cout << a << " matches " << re_str << " ? " << regex_match(a, re) << endl;
  cout << b << " matches " << re_str << " ? " << regex_match(b, re) << endl;
  cout << c << " matches " << re_str << " ? " << regex_match(c, re) << endl;
  cout << d << " matches " << re_str << " ? " << regex_match(d, re) << endl;
  cout << e << " matches " << re_str << " ? " << regex_match(e, re) << endl;

  string re_2_str = "(argh)+(argh)+";
  regex re_2(re_2_str);
  cout << e << " matches " << re_2_str << " ? " << regex_match(e, re_2) << endl;

  Rule a_rule("123: \"a\"");
  Rule other_rule("123: 123 456 | 789 987 65 | 3");
  cout << a_rule << endl;
  cout << other_rule << endl;
  cout << "******************** END *********************" << endl;
}

int main() {
  using namespace match_utils;
  Test();
  string line;
  Ruleset rules;
  while (getline(cin, line)) {
    if (line == "") break;
    rules.AddRule(move(Rule(line)));
  }

  regex regex_full(rules.GetRegexForRule(0));
  regex regex_42(rules.GetRegexForRule(42));
  regex regex_31(rules.GetRegexForRule(31));
  // examine messages
  int count_valid_part1 = 0;
  int count_valid_part2 = 0;
  while (getline(cin, line)) {
    if (regex_match(line, regex_full)) ++count_valid_part1;

    // for the second part:
    // rule 0: 8 11 and new rules are 8: 42 | 42 8 and 11: 42 31 | 42 11 31
    // this means to match the new rules we need to have 2+ of 42 and 1+ of 31,
    // but also there must be more matches of 42 than 31
    if (MatchesMoreAsThanBsWhenFullyMatched(line, regex_42, regex_31)) {
      ++count_valid_part2;
    }
  }
  cout << "Part 1: " << count_valid_part1 << endl;
  cout << "Part 2: " << count_valid_part2 << endl;
}
