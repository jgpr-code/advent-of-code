#include <iostream>
#include <string>
#include <deque>
#include <unordered_set>

using namespace std;
using ll = long long;

deque<ll>& play_the_game(deque<ll>& deck_player_one, deque<ll>& deck_player_two) {
    while (!(deck_player_one.empty() || deck_player_two.empty())) {
        ll card_player_one = deck_player_one.front();
        ll card_player_two = deck_player_two.front();
        deck_player_one.pop_front();
        deck_player_two.pop_front();
        //cout << "Round: " << card_player_one << " vs " << card_player_two << endl;

        if (card_player_one > card_player_two) {
            deck_player_one.push_back(card_player_one);
            deck_player_one.push_back(card_player_two);
        } else {
            deck_player_two.push_back(card_player_two);
            deck_player_two.push_back(card_player_one);
        }
    }
    return deck_player_two.empty() ? deck_player_one : deck_player_two;
}

string game_state(const deque<ll>& deck_player_one, const deque<ll>& deck_player_two) {
    string state = "";
    for (auto card : deck_player_one) {
        state += to_string(card) + ",";
    }
    state += "|";
    for (auto card : deck_player_two) {
        state += to_string(card) + ",";
    }
    return state;
}

deque<ll> get_sub_deck(ll amount, const deque<ll>& deck_to_pull_from) {
    deque<ll> sub_deck;
    for (auto it = deck_to_pull_from.cbegin(); it != deck_to_pull_from.cend(); ++it) {
        sub_deck.push_back(*it);
        --amount;
        if (amount == 0) {
            break;
        }
    }
    return sub_deck;
}

enum class Winner {
    PLAYER_ONE,
    PLAYER_TWO
};

Winner play_recursive(deque<ll>& deck_player_one, deque<ll>& deck_player_two) {
    unordered_set<string> encountered_game_states;
    while (!(deck_player_one.empty() || deck_player_two.empty())) {
        string current_game_state = game_state(deck_player_one, deck_player_two);
        if (encountered_game_states.count(current_game_state) == 1) {
            return Winner::PLAYER_ONE;
        }
        encountered_game_states.insert(current_game_state);
        ll card_player_one = deck_player_one.front();
        ll card_player_two = deck_player_two.front();
        deck_player_one.pop_front();
        deck_player_two.pop_front();
        //cout << "Round: " << card_player_one << " vs " << card_player_two << endl;
        auto winner = Winner::PLAYER_ONE;
        if (static_cast<ll>(deck_player_one.size()) >= card_player_one &&
            static_cast<ll>(deck_player_two.size()) >= card_player_two) {
            deque<ll> sub_deck_player_one = get_sub_deck(card_player_one, deck_player_one);
            deque<ll> sub_deck_player_two = get_sub_deck(card_player_two, deck_player_two);
            winner = play_recursive(sub_deck_player_one, sub_deck_player_two);
        } else if (card_player_one > card_player_two) {
            winner = Winner::PLAYER_ONE;
        } else {
            winner = Winner::PLAYER_TWO;
        }
        if (winner == Winner::PLAYER_ONE) {
            deck_player_one.push_back(card_player_one);
            deck_player_one.push_back(card_player_two);
        } else {
            deck_player_two.push_back(card_player_two);
            deck_player_two.push_back(card_player_one);
        }
    }
    return deck_player_two.empty() ? Winner::PLAYER_ONE : Winner::PLAYER_TWO;
}

ll score_winning_deck(deque<ll>& deck) {
    ll score = 0;
    ll score_multiplier = 1;
    for (auto it = deck.rbegin(); it != deck.rend(); ++it) {
        score += score_multiplier * *it;
        ++score_multiplier;
    }
    return score;
}

void Unit_Tests();

int main() {
    Unit_Tests();

    deque<ll> deck_player_one;
    deque<ll> deck_player_two;
    auto* current_deck = &deck_player_one;
    for (string line; getline(cin, line);) {
        if (line == "") {
            current_deck = &deck_player_two;
        }
        try {
            ll card = stoll(line);
            current_deck->push_back(card);
        } catch (const std::exception&) {
            cerr << "Ignoring " << line << endl;
        }
    }
    cout << "Deck 1:";
    for (auto card : deck_player_one) {
        cout << " " << card;
    }
    cout << endl;

    cout << "Deck 2:";
    for (auto card : deck_player_two) {
        cout << " " << card;
    }
    cout << endl;
    deque<ll> deck_player_one_part2 = deck_player_one;
    deque<ll> deck_player_two_part2 = deck_player_two;

    auto& winning_deck = play_the_game(deck_player_one, deck_player_two);
    cout << "Part 1: " << score_winning_deck(winning_deck) << endl;

    auto winner = play_recursive(deck_player_one_part2, deck_player_two_part2);
    winning_deck = (winner == Winner::PLAYER_ONE) ? deck_player_one_part2 : deck_player_two_part2;
    cout << "Part 2: " << score_winning_deck(winning_deck) << endl;
}

void Unit_Tests() {
    deque<ll> deck_player_one = { 9, 2, 6, 3, 1 };
    deque<ll> deck_player_two = { 5, 8, 4, 7, 10 };

    auto& winning_deck = play_the_game(deck_player_one, deck_player_two);
    cout << "Winning deck: ";
    for (const auto& card : winning_deck) {
        cout << " " << card;
    }
    cout << endl;
    cout << "Score: " << score_winning_deck(winning_deck) << endl;

    deck_player_one = { 9, 2, 6, 3, 1 };
    deck_player_two = { 5, 8, 4, 7, 10 };

    auto winner = play_recursive(deck_player_one, deck_player_two);
    winning_deck = (winner == Winner::PLAYER_ONE) ? deck_player_one : deck_player_two;
    cout << "Winning deck: ";
    for (const auto& card : winning_deck) {
        cout << " " << card;
    }
    cout << endl;
    cout << "Score: " << score_winning_deck(winning_deck) << endl;
}