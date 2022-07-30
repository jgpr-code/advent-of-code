#include <iostream>
#include <string>
#include <list>
#include <set>
#include <algorithm>
#include <unordered_map>

using namespace std;
using ll = long long;

struct FastCupSimulator {
    bool myShowSteps;
    list<ll>::const_iterator myCurrentCup;
    list<ll> myCups;
    unordered_map<ll, list<ll>::const_iterator> myCupsIndex;
    ll myNumberOfCups;

    FastCupSimulator(ll nCups, const list<ll>& firstCups) {
        myShowSteps = false;
        ll maxFirstCup = 1;
        for (auto it = firstCups.cbegin(); it != firstCups.cend(); ++it) {
            ll cup = *it;
            auto dstIt = myCups.insert(myCups.end(), cup);
            myCupsIndex.emplace(cup, dstIt);
            maxFirstCup = max(cup, maxFirstCup);
        }
        for (ll cup = maxFirstCup + 1; cup <= nCups; ++cup) {
            auto dstIt = myCups.insert(myCups.end(), cup);
            myCupsIndex.emplace(cup, dstIt);
        }
        myCurrentCup = myCups.begin();
        myNumberOfCups = nCups;
    }

    list<ll>::const_iterator next(list<ll>::const_iterator it) const {
        if (it == myCups.cend()) { // end should never be reached unless the list is empty
            return it;
        }
        ++it;
        if (it == myCups.cend()) {
            it = myCups.cbegin();
        }
        return it;
    }

    list<ll> pick_up(list<ll>::const_iterator it) {
        list<ll> picked_up;
        it = next(it);
        for (ll i = 0; i < 3; ++i) {
            picked_up.push_back(*it);
            auto erase_it = it;
            it = next(it);
            myCups.erase(erase_it);
        }

        return picked_up;
    }

    ll wrapping_decrement(ll num) const {
        ll ret = num - 1;
        if (ret < 1) {
            ret = myNumberOfCups;
        }
        return ret;
    }

    list<ll>::const_iterator find_destination_cup(ll current_cup, const list<ll> picked_up_cups) {
        set<ll> picked_up_set(picked_up_cups.begin(), picked_up_cups.end());
        ll target = wrapping_decrement(current_cup);
        while (picked_up_set.count(target) == 1) {
            target = wrapping_decrement(target);
        }
        auto destination = myCupsIndex[target];
        return next(destination);
    }

    void make_move() {
        auto current_cup = *myCurrentCup;
        auto picked_up_cups = pick_up(myCurrentCup);
        auto dest = find_destination_cup(current_cup, picked_up_cups);
        for (auto cup : picked_up_cups) {
            auto dstIt = myCups.insert(dest, cup);
            myCupsIndex[cup] = dstIt;
        }
        myCurrentCup = next(myCurrentCup);
    }

    void print_cups_normalized() const {
        auto cup_one = myCupsIndex.at(1);
        for (auto it = next(cup_one); it != cup_one; it = next(it)) {
            cout << *it;
        }
        cout << endl;
    }

    void print_cups() const {
        cout << "cups:";
        for (auto it = myCups.begin(); it != myCups.end(); ++it) {
            cout << " ";
            if (it == myCurrentCup) {
                cout << "(" << *it << ")";
            } else {
                cout << *it;
            }
        }
        cout << endl;
    }

    void simulateMoves(ll amount) {
        if (myShowSteps) {
            print_cups();
        }
        for (ll i = 0; i < amount; ++i) {
            make_move();
            if (myShowSteps) {
                print_cups();
            }
        }
    }

    ll partTwo() const {
        auto firstAfterOne = next(myCupsIndex.at(1));
        auto secondAfterOne = next(firstAfterOne);
        return *firstAfterOne * *secondAfterOne;
    }
};

int main() {
    list<ll> inputCups = { 4, 6, 7, 5, 2, 8, 1, 9, 3 };
    list<ll> testCups = { 3, 8, 9, 1, 2, 5, 4, 6, 7 };
    FastCupSimulator testSimulator(9, testCups);
    testSimulator.myShowSteps = true;
    testSimulator.simulateMoves(10);
    testSimulator.print_cups_normalized();

    FastCupSimulator inputSimulator(9, inputCups);
    inputSimulator.simulateMoves(100);
    cout << "Part 1: ";
    inputSimulator.print_cups_normalized();

    FastCupSimulator largeInputSimulator(1000000, inputCups);
    largeInputSimulator.simulateMoves(10000000);
    cout << "Part 2: " << largeInputSimulator.partTwo() << endl;
}