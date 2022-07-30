#include <iostream>
#include <map>
#include <regex>
#include <set>
#include <sstream>
#include <string>
#include <vector>
using namespace std;
class Passport {
 private:
  static const set<string> requiredEntries;
  map<string, string> entries;

 public:
  Passport(const string& passport) {
    istringstream passportStream(passport);
    for (string entry; getline(passportStream >> ws, entry, ' ');) {
      istringstream entryStream(entry);
      string entryName;
      string entryValue;
      getline(entryStream >> ws, entryName, ':');
      getline(entryStream >> ws, entryValue);
      entries.emplace(entryName, entryValue);
    }
  }
  bool isValid() {
    for (const auto& req : requiredEntries) {
      if (entries.count(req) == 0) return false;
    }
    return true;
  }
  bool isValidByr(const string& byr) {
    regex rg("^[0-9]{4}$");
    if (!regex_match(byr, rg)) return false;
    istringstream ss(byr);
    int year;
    ss >> year;
    return 1920 <= year && year <= 2002;
  }
  bool isValidIyr(const string& iyr) {
    regex rg("^[0-9]{4}$");
    if (!regex_match(iyr, rg)) return false;
    istringstream ss(iyr);
    int year;
    ss >> year;
    return 2010 <= year && year <= 2020;
  }
  bool isValidEyr(const string& eyr) {
    regex rg("^[0-9]{4}$");
    if (!regex_match(eyr, rg)) return false;
    istringstream ss(eyr);
    int year;
    ss >> year;
    return 2020 <= year && year <= 2030;
  }
  bool isValidHgt(const string& hgt) {
    regex rgcm("^[0-9]{3}cm$");
    regex rgin("^[0-9]{2}in$");
    if (regex_match(hgt, rgcm)) {
      istringstream ss(hgt);
      int height;
      ss >> height;
      return 150 <= height && height <= 193;
    } else if (regex_match(hgt, rgin)) {
      istringstream ss(hgt);
      int height;
      ss >> height;
      return 59 <= height && height <= 76;
    } else {
      return false;
    }
  }
  bool isValidHcl(const string& hcl) {
    regex rg("^#[0-9a-f]{6}$");
    return regex_match(hcl, rg);
  }
  bool isValidEcl(const string& ecl) {
    set<string> validColors = {"amb", "blu", "brn", "gry", "grn", "hzl", "oth"};
    return validColors.count(ecl) > 0;
  }
  bool isValidPid(const string& pid) {
    regex rg("^[0-9]{9}$");
    return regex_match(pid, rg);
  }
  bool isStrictlyValid() {
    if (!isValid()) return false;
    if (!isValidByr(entries["byr"])) return false;
    if (!isValidIyr(entries["iyr"])) return false;
    if (!isValidEyr(entries["eyr"])) return false;
    if (!isValidHgt(entries["hgt"])) return false;
    if (!isValidHcl(entries["hcl"])) return false;
    if (!isValidEcl(entries["ecl"])) return false;
    if (!isValidPid(entries["pid"])) return false;
    return true;
  }
};
const set<string> Passport::requiredEntries = {
    "byr",  // Birth Year
    "iyr",  // Issue Year
    "eyr",  // Expiration Year
    "hgt",  // Height
    "hcl",  // Hair Color
    "ecl",  // Eye Color
    "pid"   // Passport ID
};
int main() {
  stringstream ss("0020cm");
  int test;
  ss >> test;
  cout << test << endl;
  vector<string> passports = {""};
  for (string line; getline(cin, line);) {
    if (line.compare("") == 0) {  // this means we encountered a blank line
      passports.push_back("");    // init next passport
    } else {
      passports.back().append(" " + line);
    }
  }
  int countValidPassports = 0;
  int countStrictlyValidPassports = 0;
  for (string passport : passports) {
    Passport currentPassport(passport);
    if (currentPassport.isValid()) ++countValidPassports;
    if (currentPassport.isStrictlyValid()) ++countStrictlyValidPassports;
  }
  cout << "Part 1: " << countValidPassports << endl;  // 216
  cout << "Part 2: " << countStrictlyValidPassports << endl;
}
