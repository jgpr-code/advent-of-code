#include <iostream>
#include <string>
#include <regex>
#include <map>
#include <set>
#include <deque>

using namespace std;

vector<string> split_string(const string& str, char separator) {
    vector<string> ret;
    size_t begin_search_at = 0;
    while (true) {
        size_t separator_pos = str.find(separator, begin_search_at);
        ret.push_back(str.substr(begin_search_at, separator_pos - begin_search_at));
        if (separator_pos == string::npos) {
            return ret;
        }
        begin_search_at = separator_pos + 1;
    }
    return ret;
}

struct Food {
    vector<string> my_ingredients;
    vector<string> my_allergenes;
    Food(const vector<string>& ingredients, const vector<string>& allergenes)
        : my_ingredients(ingredients), my_allergenes(allergenes) {}
};

int main() {
    vector<Food> foods;
    map<string, set<string>> allergene_to_ingredients;
    const regex food_regex("^(.*) \\(contains (.*)\\)$");
    for (string line; getline(cin, line);) {
        smatch matches;
        regex_match(line, matches, food_regex);
        string matched_ingredients = matches[1].str();
        string matched_allergenes = matches[2].str();
        matched_allergenes.erase(remove(matched_allergenes.begin(), matched_allergenes.end(), ' '), matched_allergenes.end());
        vector<string> ingredients = split_string(matched_ingredients, ' ');
        vector<string> allergenes = split_string(matched_allergenes, ',');
        foods.emplace_back(ingredients, allergenes);
        for (string allergene : allergenes) {
            if (allergene_to_ingredients.count(allergene) == 0) {
                allergene_to_ingredients.emplace(allergene, set<string>(ingredients.begin(), ingredients.end()));
            } else {
                set<string> updated_ingredients;
                set<string> current_ingredients(ingredients.begin(), ingredients.end());
                for (string ingredient : current_ingredients) {
                    if (allergene_to_ingredients[allergene].count(ingredient) == 1) {
                        updated_ingredients.insert(ingredient);
                    }
                }
                allergene_to_ingredients[allergene] = updated_ingredients;
            }
        }
    }
    map<string, string> ingredient_to_allergene;
    deque<string> allergene_queue;
    for (const auto& kvp : allergene_to_ingredients) {
        if (kvp.second.size() == 1) {
            allergene_queue.push_back(kvp.first);
        }
    }
    while (!allergene_queue.empty()) {
        string resolved_allergene = allergene_queue.front();
        set<string> resolved_options = allergene_to_ingredients[resolved_allergene];
        if (resolved_options.size() < 1) {
            continue;
        }
        if (resolved_options.size() > 1) {
            cerr << "ERROR: resolved allergene was not resolved" << endl;
        }
        string resolved_ingredient = *resolved_options.begin();
        ingredient_to_allergene.emplace(resolved_ingredient, resolved_allergene);
        for (auto& kvp : allergene_to_ingredients) {
            kvp.second.erase(resolved_ingredient);
            if (kvp.second.size() == 1) {
                allergene_queue.push_back(kvp.first);
            }
        }
        allergene_queue.pop_front();
    }
    for (const auto& kvp : ingredient_to_allergene) {
        cout << kvp.first << " -> " << kvp.second << endl;
    }

    size_t count_safe_ingredients = 0;
    for (const auto& food : foods) {
        for (const auto& ingredient : food.my_ingredients) {
            if (ingredient_to_allergene.count(ingredient) == 0) {
                ++count_safe_ingredients;
            }
        }
    }

    cout << "Part 1: " << count_safe_ingredients << endl;
    cout << "Part 2: ";
    // map is sorted by the key by default
    map<string, string> allergene_to_ingredient;
    for (const auto& kvp : ingredient_to_allergene) {
        allergene_to_ingredient.emplace(kvp.second, kvp.first);
    }
    for (const auto& kvp : allergene_to_ingredient) {
        cout << kvp.second << ",";
    }

    // insert or update into map[allergene] -> {ingredients} keeping only the ingredients that appear in both (current and new)
    // sort map by length of values
    // queue all with length = 1
    // while !empty get the resolved ingredient -> allergene
    // iterate through map removing the resolved
    // queuing new elements with length 1
}

/*
mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)

ingredients >> allergenes

algorithm outline:
for each allergene find ingredients that are listed for every food listing the allergene
convert to allergene -> list of possible ingredients
sort and subsequently resolve

dairy: mxmxvkd
fish: mxmxvkd, sqjhc
soy: sqjhc, fvjkl


mxmxvkd  dairy

*/