#include <iostream>
#include <string>
#include <unordered_set>
#include <vector>
using namespace std;
class GroupAnswer {
  unordered_set<char> allGivenAnswers;
  vector<unordered_set<char>> personAnswers;

 public:
  void addAnswersFromPerson(const string& answers) {
    allGivenAnswers.insert(answers.begin(), answers.end());
    personAnswers.emplace_back(answers.begin(), answers.end());
  }
  int countAnswered() const { return allGivenAnswers.size(); }
  int countAnsweredByAll() const {
    int count = 0;
    for (char answer : allGivenAnswers) {
      bool answeredByAll = true;
      for (const auto& personAnswer : personAnswers) {
        if (personAnswer.count(answer) == 0) {
          answeredByAll = false;
          break;
        }
      }
      if (answeredByAll) ++count;
    }
    return count;
  }
};
int main() {
  vector<GroupAnswer> groupAnswers;
  GroupAnswer currentGroup;
  for (string line; getline(cin, line);) {
    if (line == "") {  // current group ends
      groupAnswers.push_back(currentGroup);
      currentGroup = GroupAnswer();
      continue;
    }
    currentGroup.addAnswersFromPerson(line);
  }
  int sumOfAnswered = 0;
  int sumOfAnsweredByAll = 0;
  for (const auto& groupAnswer : groupAnswers) {
    sumOfAnswered += groupAnswer.countAnswered();
    sumOfAnsweredByAll += groupAnswer.countAnsweredByAll();
  }

  // NOTE:
  // for this to work you have to insert two blank lines
  // at the end of the input!!!
  // maybe there are different getline implementations!

  cout << "Part 1: " << sumOfAnswered << endl;
  cout << "Part 2: " << sumOfAnsweredByAll << endl;
}
