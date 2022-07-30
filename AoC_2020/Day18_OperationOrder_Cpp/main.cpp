#include <algorithm>
#include <functional>
#include <iostream>
#include <regex>

using namespace std;
using ll = long long;

// Regex explanation:
// r(  \(  (  [^()]+  )  \)  )r
// r( and )r just delimit the raw string literal and are not part of the regex
// itself
// \( and \) this is used to match the literal chars '(' and ')'
// ( and ) this is to capture anything between the actual parentheses (see
// previous line)
// [^()]+ match non empty sequences that are not allowed to contain
// parentheses
static const regex innermostParensRegex(R"r(\(([^()]+)\))r",
                                        regex_constants::extended);

static const regex addition(R"r(([0-9]+\+[0-9]+))r", regex_constants::extended);

// continously replace regex match with replace_function as long as possible
string fixpointRegexReplace(const string& expression,
                            const regex& regular_expression,
                            function<string(const string&)> replace_function) {
  string expr = expression;
  smatch matches;
  while (regex_search(expr, matches, regular_expression)) {
    expr.replace(matches[0].first, matches[0].second,
                 replace_function(matches[1].str()));
  }
  return expr;
}

// slightly modified find_first_of that just returns the size of the string on
// failure
size_t findFirstOf(const string& expr, const string& search, size_t from) {
  size_t pos = expr.find_first_of(search, from);
  if (pos == string::npos) pos = expr.length();
  return pos;
}
// range based substr, to is exclusive
string substr(const string& str, size_t from, size_t to) {
  return str.substr(from, to - from);
}

ll eval_noparen_noprece(const string& expr) {
  size_t pos = findFirstOf(expr, "+*", 0);
  ll acc = stoll(substr(expr, 0, pos));
  while (pos < expr.size()) {
    size_t next_pos = findFirstOf(expr, "+*", pos + 1);
    ll operand = stoll(substr(expr, pos + 1, next_pos));
    switch (expr[pos]) {
      case '+':
        acc += operand;
        break;
      case '*':
        acc *= operand;
        break;
    }
    pos = next_pos;
  }
  return acc;
}

ll eval_noprece(const string& expression) {
  string expr = fixpointRegexReplace(
      expression, innermostParensRegex,
      [](const string& e) { return to_string(eval_noparen_noprece(e)); });
  return eval_noparen_noprece(expr);
}

ll eval_noparen_plus(const string& expression) {
  string expr = fixpointRegexReplace(expression, addition, [](const string& e) {
    return to_string(eval_noparen_noprece(e));
  });
  return eval_noparen_noprece(expr);
}

ll eval_plus(const string& expression) {
  string expr = fixpointRegexReplace(
      expression, innermostParensRegex,
      [](const string& e) { return to_string(eval_noparen_plus(e)); });
  return eval_noparen_plus(expr);
}

int main() {
  string expr("10+32*6+21*4");

  cout << eval_plus(expr) << endl;
  cout << "eval " << expr << " = " << eval_noparen_noprece(expr) << endl;

  ll exprSum = 0;
  ll exprSum2 = 0;
  for (string line; getline(cin, line);) {
    // discard whitespace
    line.erase(
        remove_if(line.begin(), line.end(), [](char c) { return isspace(c); }),
        line.end());
    exprSum += eval_noprece(line);
    exprSum2 += eval_plus(line);
  }

  cout << "Part 1: " << exprSum << endl;
  cout << "Part 2: " << exprSum2 << endl;
}
