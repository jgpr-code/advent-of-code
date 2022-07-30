#include <iostream>
#include <regex>
#include <unordered_set>
#include <vector>

using namespace std;

class HandheldProgram {
  int accumulator;
  int nextInstruction;
  vector<pair<string, int>> code;
  unordered_set<int> executedInstructions;
  vector<int> possibleFaultyInstructions;

  bool isFinished() const { return nextInstruction >= code.size(); }

  void swapJmpAndNop(int instruction) {
    string command = code[instruction].first;
    if (command == "jmp")
      code[instruction].first = "nop";
    else if (command == "nop")
      code[instruction].first = "jmp";
  }

  void executeNextInstruction() {
    const auto& p = code[nextInstruction];
    string command = p.first;
    int operand = p.second;
    if (command == "acc") {
      acc(operand);
    } else if (command == "jmp") {
      jmp(operand);
    } else if (command == "nop") {
      nop(operand);
    }
  }

  void acc(int operand) {
    accumulator += operand;
    ++nextInstruction;
  }

  void jmp(int operand) { nextInstruction += operand; }

  void nop(int) { ++nextInstruction; }

  void reset() {
    accumulator = 0;
    nextInstruction = 0;
    executedInstructions.clear();
  }

 public:
  HandheldProgram() : accumulator(0), nextInstruction(0) {}

  void appendInstruction(const string& instruction) {
    regex re("^(acc|jmp|nop) ([+-][0-9]+)$");
    smatch matches;
    if (!regex_match(instruction, matches, re)) {
      throw invalid_argument("instruction is not supported: " + instruction);
    }
    string command = matches[1].str();
    int operand = stoi(matches[2].str());
    code.push_back(make_pair(command, operand));
    if (command == "jmp" || command == "nop") {
      possibleFaultyInstructions.push_back(code.size() - 1);
    }
  }

  int getAccumulator() const { return accumulator; }

  int getCandidates() const { return possibleFaultyInstructions.size(); }

  bool runToRepeatedInstruction() {
    reset();
    while (!isFinished() && executedInstructions.count(nextInstruction) == 0) {
      executedInstructions.insert(nextInstruction);
      executeNextInstruction();
    }
    return isFinished();
  }

  void runUntilCorrect() {
    reset();
    for (int toFix : possibleFaultyInstructions) {
      swapJmpAndNop(toFix);
      if (runToRepeatedInstruction()) break;
      reset();
      swapJmpAndNop(toFix);
    }
  }
};

int main() {
  HandheldProgram program;
  for (string line; getline(cin, line);) {
    program.appendInstruction(line);
  }
  cout << program.getCandidates() << endl;
  program.runToRepeatedInstruction();
  cout << "Part 1: " << program.getAccumulator() << endl;
  program.runUntilCorrect();
  cout << "Part 2: " << program.getAccumulator() << endl;
}