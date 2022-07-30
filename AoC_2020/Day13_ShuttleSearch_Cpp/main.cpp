#include <algorithm>
#include <iostream>
#include <sstream>
#include <string>
#include <tuple>
#include <vector>

using namespace std;
using ll = long long;

ll firstBiggerMultiple(ll increment, ll target) {
  ll multiple = 0;
  while (multiple < target) {
    multiple += increment;
  }
  return multiple;
}

bool isPrime(ll n) {
  if (n <= 3) {
    return n > 1;
  }
  if (n % 2 == 0 || n % 3 == 0) {
    return false;
  }
  ll i = 5;
  while (i * i <= n) {
    if (n % i == 0 || n % (i + 2) == 0) {
      return false;
    }
    i += 6;
  }
  return true;
}

ll mod(ll a, ll b) {  // assumes b > 0
  ll r = a % b;
  return r < 0 ? r + b : r;
}

ll modmul(ll a, ll b, ll m) {
  a = mod(a, m);
  b = mod(b, m);
  ll ret = 0;
  while (b > 0) {
    if (b % 2 == 1) {
      ret = mod(ret + a, m);
    }
    a = mod(a * 2, m);
    b /= 2;
  }
  return mod(ret, m);
}

tuple<ll, ll, ll> extendedGcd(ll a, ll b) {
  auto [old_r, r] = make_tuple(a, b);
  auto [old_s, s] = make_tuple(1ll, 0ll);
  auto [old_t, t] = make_tuple(0ll, 1ll);
  while (r != 0) {
    ll quotient = old_r / r;
    tie(old_r, r) = make_tuple(r, old_r - quotient * r);
    tie(old_s, s) = make_tuple(s, old_s - quotient * s);
    tie(old_t, t) = make_tuple(t, old_t - quotient * t);
  }
  return make_tuple(old_r, old_s, old_t);  // s.t. a * old_s + b * old_t = old_r
}

tuple<ll, ll> solveTwoCongruences(tuple<ll, ll> c1, tuple<ll, ll> c2) {
  cout << "------ solveTwo ------" << endl;
  auto [a1, n1] = c1;
  auto [a2, n2] = c2;
  cout << "(" << a1 << ", " << n1 << ") and (" << a2 << ", " << n2 << ")"
       << endl;
  auto [_, m1, m2] = extendedGcd(n1, n2);
  cout << "(" << a1 << ", " << m2 << ", " << n2 << ")" << endl;
  cout << "(" << a2 << ", " << m1 << ", " << n1 << ")" << endl;
  ll m = n1 * n2;
  ll solution = mod(
      modmul(modmul(m2, n2, m), a1, m) + modmul(modmul(m1, n1, m), a2, m), m);
  if (solution < 0) solution += m;
  cout << "next congruence: (" << solution << ", " << m << ")" << endl;
  return make_tuple(solution, m);
}

ll solveCongruences(vector<tuple<ll, ll>> congruences) {
  auto acc = make_tuple(0ll, 1ll);
  for (const auto c : congruences) {
    if (get<1>(acc) > get<1>(c))
      acc = solveTwoCongruences(acc, c);
    else
      acc = solveTwoCongruences(c, acc);
  }
  auto [solution, _] = acc;
  return solution;
}

bool isSolution(ll solution, const vector<tuple<ll, ll>> &congruences) {
  if (solution <= 0) return false;
  for (const auto c : congruences) {
    auto [remainder, modulus] = c;
    if (solution % modulus != remainder) return false;
  }
  return true;
}

void unit_tests() {
  cout << "UNIT TESTS" << endl;
  {
    auto [gcd, s, t] = extendedGcd(853, 523);
    cout << "gcd: " << gcd << endl;
    cout << "s: " << s << endl;
    cout << "t: " << t << endl;
    cout << "as + bt = " << 853 * s + 523 * t << endl;
  }
  {
    auto c2 = make_tuple(0ll, 1ll);
    auto c1 = make_tuple(2ll, 3ll);
    auto [solution, _] = solveTwoCongruences(c1, c2);
    cout << "(0, 1) and (2, 3) = " << solution << endl;
  }

  {
    vector<tuple<ll, ll>> cs = {make_tuple(0ll, 3ll), make_tuple(3ll, 4ll),
                                make_tuple(4ll, 5ll)};
    cout << "solve (0,3) (3,4) (4,5) = " << solveCongruences(cs) << endl;
  }
  {
    vector<tuple<ll, ll>> cs = {make_tuple(0ll, 1789ll), make_tuple(36ll, 37ll),
                                make_tuple(45, 47), make_tuple(1886, 1889)};
    ll sol = solveCongruences(cs);
    if (sol != 1202161486) cout << "diff to sol: " << sol - 1202161486 << endl;
    cout << "actual: " << sol << endl;
  }
  cout << "--------------------------------------------------------------------"
          "------"
       << endl;
}

int main() {
  unit_tests();
  string line;
  getline(cin, line);
  ll targetDeparture = stoll(line);

  tuple<ll, ll> bestBus(numeric_limits<ll>::max(), -1);
  vector<tuple<ll, ll>> busCongruences;

  getline(cin, line);
  istringstream iss(line);
  string token;
  for (ll i = 0; getline(iss, token, ','); ++i) {
    if (token == "x") {
      continue;
    }
    ll busId = stoll(token);
    ll busDeparture = firstBiggerMultiple(busId, targetDeparture);
    bestBus = min(bestBus, make_tuple(busDeparture, busId));
    cout << "busId = " << busId << ", i = " << i << endl;
    busCongruences.push_back(make_tuple(mod(busId - i, busId), busId));
  }
  for (auto c : busCongruences) {
    cout << "(" << get<0>(c) << ", " << get<1>(c) << ")" << endl;
  }

  auto [bestDeparture, bestId] = bestBus;
  ll waitTime = bestDeparture - targetDeparture;
  cout << "Part 1: " << bestId * waitTime << endl;

  ll t = solveCongruences(busCongruences);
  if (!isSolution(t, busCongruences)) cout << "Wrong solution" << endl;
  cout << "Part 2: " << t << endl;
}