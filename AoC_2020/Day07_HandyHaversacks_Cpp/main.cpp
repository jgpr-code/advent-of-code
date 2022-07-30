#include <iostream>
#include <regex>
#include <sstream>
#include <string>
#include <unordered_map>
#include <unordered_set>
#include <vector>
using namespace std;
class BagsGraph {
  unordered_map<string, vector<pair<int, string>>> containsAdjacencyList;
  unordered_map<string, vector<string>> insideOfAdjacencyList;
  unordered_set<string> visited;
  void dfsInside(const string& node) {
    visited.insert(node);
    for (const auto& neighbour : insideOfAdjacencyList[node]) {
      if (visited.count(neighbour) == 0) dfsInside(neighbour);
    }
  }

 public:
  void addLine(const string& line) {
    regex re("^(.*) bags contain (.*)$");
    smatch matches;
    if (!regex_match(line, matches, re)) return;
    string holdingBag = matches[1].str();
    containsAdjacencyList.emplace(holdingBag, vector<pair<int, string>>());
    string rest = matches[2].str();
    istringstream iss(rest);
    for (string bagStr; getline(iss >> ws, bagStr, ',');) {
      re = "^([1-9][0-9]*) (.*) bag.*$";
      if (!regex_match(bagStr, matches, re)) continue;
      int amount = stoi(matches[1].str());
      string containedBag = matches[2].str();
      insideOfAdjacencyList.emplace(containedBag, vector<string>());
      containsAdjacencyList[holdingBag].push_back(
          make_pair(amount, containedBag));
      insideOfAdjacencyList[containedBag].push_back(holdingBag);
    }
  }
  void clearVisited() { visited.clear(); }
  int countInsideDfs(const string& bag) {
    int amount = 1;
    visited.insert(bag);
    for (const auto& containingBag : insideOfAdjacencyList[bag]) {
      if (visited.count(containingBag) == 0) {
        amount += countInsideDfs(containingBag);
      }
    }
    return amount;
  }
  int countContainedDAG(const string& bag) {
    int amount = 1;
    for (const auto& containedBagPair : containsAdjacencyList[bag]) {
      int amountContained = containedBagPair.first;
      string containedBag = containedBagPair.second;
      amount += amountContained * countContainedDAG(containedBag);
    }
    return amount;
  }
};
int main() {
  BagsGraph graph;
  for (string line; getline(cin, line);) {
    graph.addLine(line);
  }
  cout << "Part 1: " << graph.countInsideDfs("shiny gold") - 1 << endl;
  graph.clearVisited();
  cout << "Part 2: " << graph.countContainedDAG("shiny gold") - 1 << endl;
}