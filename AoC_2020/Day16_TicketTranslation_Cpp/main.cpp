#include <algorithm>
#include <cassert>
#include <iostream>
#include <regex>
#include <set>
#include <sstream>
#include <string>
#include <vector>

using namespace std;

using ll = long long;

// try out designated initializers (c++20)

struct Range {
  ll low;
  ll high;
};

struct FieldRanges {
  Range low_range;
  Range high_range;
};

struct TicketField {
  string name;
  FieldRanges ranges;
};

struct TicketSpecification {
  vector<TicketField> allowed_fields;
};

struct Ticket {
  vector<ll> fields;
};

TicketField parseTicketField(const string& line) {
  string capture_name = "(.*)";
  string capture_range = "([[:digit:]]+)-([[:digit:]]+)";
  regex ticket_field_regex(capture_name + ": " + capture_range + " or " +
                           capture_range);
  smatch matches;
  regex_match(line, matches, ticket_field_regex);
  return {.name = matches[1].str(),
          .ranges = {.low_range = {.low = stoll(matches[2].str()),
                                   .high = stoll(matches[3].str())},
                     .high_range = {.low = stoll(matches[4].str()),
                                    .high = stoll(matches[5].str())}}};
}

Ticket parseTicket(const string& line) {
  vector<ll> fields;
  istringstream iss(line);
  for (string num; getline(iss, num, ',');) {
    fields.push_back(stoll(num));
  }
  return {.fields = move(fields)};
}

bool isInRange(ll num, const Range& range) {
  return num >= range.low && num <= range.high;
}

bool isInFieldRanges(ll num, const FieldRanges& ranges) {
  return isInRange(num, ranges.low_range) || isInRange(num, ranges.high_range);
}

bool isValidForField(ll num, const TicketField& field) {
  return isInFieldRanges(num, field.ranges);
}

bool isValidForOneOrMoreFields(ll num,
                               const TicketSpecification& specification) {
  for (const auto field : specification.allowed_fields) {
    if (isValidForField(num, field)) return true;
  }
  return false;
}

bool isTicketValid(const Ticket& ticket,
                   const TicketSpecification& specification) {
  for (const auto field : ticket.fields) {
    if (!isValidForOneOrMoreFields(field, specification)) return false;
  }
  return true;
}

ll sumInvalidFieldsForTicket(const Ticket& ticket,
                             const TicketSpecification& specification) {
  ll sum = 0;
  for (const auto field : ticket.fields) {
    if (!isValidForOneOrMoreFields(field, specification)) sum += field;
  }
  return sum;
}

void initSetFromTo(set<size_t>& s, size_t from, size_t to) {
  s.clear();
  for (size_t i = from; i < to; ++i) s.insert(i);
}

void printPossibilities(const vector<set<size_t>>& possibilities) {
  int debug_i = 0;
  for (const auto& s : possibilities) {
    cout << debug_i++ << ": " << s.size() << " -> ";
    for (auto elem : s) {
      cout << elem << ",";
    }
    cout << endl;
  }
}

void reprintInput(const TicketSpecification& spec, const vector<Ticket>& nearby,
                  const Ticket& my_ticket) {
  for (size_t i = 0; i < spec.allowed_fields.size(); ++i) {
    auto field = spec.allowed_fields[i];
    cout << field.name << ": " << field.ranges.low_range.low << "-"
         << field.ranges.low_range.high << " or " << field.ranges.high_range.low
         << "-" << field.ranges.high_range.high << endl;
  }
  cout << endl;

  cout << "your ticket:" << endl;
  for (size_t i = 0; i < my_ticket.fields.size(); ++i) {
    auto field = my_ticket.fields[i];
    if (i == 0)
      cout << field;
    else
      cout << "," << field;
  }
  cout << endl << endl;

  cout << "nearby tickets:" << endl;
  for (size_t i = 0; i < nearby.size(); ++i) {
    auto ticket = nearby[i];
    for (size_t j = 0; j < ticket.fields.size(); ++j) {
      auto field = ticket.fields[j];
      if (j == 0)
        cout << field;
      else
        cout << "," << field;
    }
    cout << endl;
  }
}

int main() {
  TicketSpecification ticket_specification;
  vector<Ticket> nearby_tickets;
  vector<Ticket> valid_tickets;
  Ticket my_ticket;
  string line;

  for (; getline(cin, line);) {
    if (line == "") break;
    ticket_specification.allowed_fields.push_back(parseTicketField(line));
  }

  getline(cin, line);
  assert(line == "your ticket:");
  getline(cin, line);
  my_ticket = parseTicket(line);
  valid_tickets.push_back(my_ticket);

  getline(cin, line);
  assert(line == "");

  getline(cin, line);
  assert(line == "nearby tickets:");
  for (; getline(cin, line);) {
    nearby_tickets.push_back(parseTicket(line));
  }

  reprintInput(ticket_specification, nearby_tickets,
               my_ticket);  // this is correct!

  ll sum_of_invalid = 0;
  for (const auto ticket : nearby_tickets) {
    if (isTicketValid(ticket, ticket_specification)) {
      valid_tickets.push_back(ticket);
    } else {
      sum_of_invalid += sumInvalidFieldsForTicket(ticket, ticket_specification);
    }
  }
  cout << "Part 1: " << sum_of_invalid << endl;

  // before scanning all fields are allowed for every position
  // goal:
  // for all idx in possible_fields_at_position[pos] :
  // ticket_specification.allowed_fields[idx] is a ticket field that could be
  // use in the position pos
  vector<set<size_t>> possible_fields_at_position(my_ticket.fields.size());

  for (size_t i = 0; i < possible_fields_at_position.size(); ++i) {
    auto& s = possible_fields_at_position[i];
    initSetFromTo(s, 0, ticket_specification.allowed_fields.size());
  }

  cout << "n allowed_fields: " << ticket_specification.allowed_fields.size()
       << endl;
  cout << "n ticket entries: " << my_ticket.fields.size() << endl;
  cout << "initial set sizes: " << endl;
  printPossibilities(possible_fields_at_position);

  size_t ticket_idx_here = 0;
  bool problem_found = false;
  for (const auto ticket : valid_tickets) {
    for (size_t pos = 0; pos < ticket.fields.size(); ++pos) {
      ll ticket_element = ticket.fields[pos];
      const auto possible_fields = possible_fields_at_position[pos];
      if (pos == 10) {
        cout << "analyze(pos=10), ticket=" << ticket_idx_here++ << ": ";
        for (const auto idx : possible_fields) {
          cout << idx << ",";
        }
        cout << endl;
      }
      for (const auto idx : possible_fields) {
        if (!isValidForField(ticket_element,
                             ticket_specification.allowed_fields[idx])) {
          possible_fields_at_position[pos].erase(idx);
        }
      }
      if (!problem_found && possible_fields_at_position[pos].empty()) {
        cout << "problematic ticket elment: " << ticket_element << endl;
        cout << "found at pos " << pos << "in ticket nr " << ticket_idx_here
             << endl;
        problem_found = true;
      }
    }
  }

  auto sorted_possible_fields_at_position = possible_fields_at_position;
  sort(sorted_possible_fields_at_position.begin(),
       sorted_possible_fields_at_position.end(),
       [](const set<size_t>& a, const set<size_t>& b) {
         return a.size() < b.size();
       });

  cout << "erased and sorted:" << endl;
  printPossibilities(sorted_possible_fields_at_position);

  vector<size_t> position_in_ticket_for_field(
      ticket_specification.allowed_fields.size(), 0);

  struct PositionPossibilities {
    size_t pos;
    set<size_t> specs_indices;
  };

  vector<PositionPossibilities> all_position_possibilities;

  for (size_t i = 0; i < possible_fields_at_position.size(); ++i) {
    const auto specs_indices = possible_fields_at_position[i];
    all_position_possibilities.push_back(
        {.pos = i, .specs_indices = specs_indices});
  }

  // sort remaining sets after their size, size == 1 -> only one possibility
  sort(all_position_possibilities.begin(), all_position_possibilities.end(),
       [](const PositionPossibilities& a, const PositionPossibilities& b) {
         return a.specs_indices.size() < b.specs_indices.size();
       });

  for (size_t i = 0; i < all_position_possibilities.size(); ++i) {
    auto& pp = all_position_possibilities[i];
    //    assert(pp.specs_indices.size() == 1);
    if (pp.specs_indices.size() < 1) continue;
    size_t idx = *pp.specs_indices.begin();
    position_in_ticket_for_field[idx] = pp.pos;
    for (size_t j = i + 1; j < all_position_possibilities.size(); ++j) {
      all_position_possibilities[j].specs_indices.erase(idx);
    }
  }

  ll ans = 1;
  for (size_t i = 0; i < position_in_ticket_for_field.size(); ++i) {
    if (ticket_specification.allowed_fields[i].name.starts_with("departure")) {
      size_t pos_in_ticket = position_in_ticket_for_field[i];
      ans *= my_ticket.fields[pos_in_ticket];
    }
  }
  cout << "Part 2: " << ans << endl;
}
