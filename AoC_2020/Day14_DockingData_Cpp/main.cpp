#include <bitset>
#include <cassert>
#include <iostream>
#include <map>
#include <sstream>
#include <string>
#include <vector>

using namespace std;
using ll = long long;

class DockerDecoder {
  static const size_t bit_length = 36;
  string internal_mask;
  map<ll, string> internal_mem;
  map<string, string> internal_mem_v2;

  static void fill_floating(size_t pos, string address,
                            vector<string> &results) {
    size_t x = address.find('X', pos);
    if (x != string::npos) {
      address[x] = '0';
      fill_floating(x + 1, address, results);
      address[x] = '1';
      fill_floating(x + 1, address, results);
    } else {
      results.push_back(address);
    }
  }

  static vector<string> resolve_floating_address(const string &address) {
    vector<string> results;
    fill_floating(0, address, results);
    return results;
  }

  string mask_value(const string &value) const {
    assert(internal_mask.size() == bit_length && value.size() == bit_length);
    string masked = value;
    for (size_t i = 0; i < bit_length; ++i) {
      if (internal_mask[i] != 'X') {
        masked[i] = internal_mask[i];
      }
    }
    return masked;
  }

  string mask_address(const string &address) const {
    assert(internal_mask.size() == bit_length && address.size() == bit_length);
    string masked = address;
    for (size_t i = 0; i < bit_length; ++i) {
      if (internal_mask[i] != '0') {
        masked[i] = internal_mask[i];
      }
    }
    return masked;
  }

public:
  static string lltobit(const string &llstr) {
    ll value = stoll(llstr);
    bitset<bit_length> bitrepresentation(value);
    return bitrepresentation.to_string();
  }

  void set_mask(const string &mask) { internal_mask = mask; }

  void update_mem(ll address, const string &value) {
    assert(value.size() == bit_length);
    internal_mem.try_emplace(address, string(bit_length, '0'));
    internal_mem[address] = mask_value(value);
  }

  void update_mem_v2(string address, const string &value) {
    assert(value.size() == bit_length && address.size() == bit_length);
    address = mask_address(address);
    vector<string> addresses = resolve_floating_address(address);
    for (string addr : addresses) {
      internal_mem_v2.try_emplace(addr, string(bit_length, '0'));
      internal_mem_v2[addr] = value;
    }
  }

  ll mem_sum() const {
    ll sum = 0;
    for (const auto &kvp : internal_mem) {
      sum += stoll(kvp.second, 0, 2);
    }
    return sum;
  }

  ll mem_sum_v2() const {
    ll sum = 0;
    for (const auto &kvp : internal_mem_v2) {
      sum += stoll(kvp.second, 0, 2);
    }
    return sum;
  }
};

int main() {
  DockerDecoder decoder;
  DockerDecoder decoder_v2;

  for (string line; getline(cin, line);) {
    assert(line.find("-") == string::npos && "Can't handle negative values");
    istringstream linestream(line);
    string lhs;
    string eqsign;
    string rhs;
    getline(linestream, lhs, ' ');
    getline(linestream, eqsign, ' ');
    getline(linestream, rhs, ' ');
    assert(eqsign == "=");
    istringstream lhsstream(lhs);
    string operation;
    getline(lhsstream, operation, '[');
    if (operation == "mask") {
      decoder.set_mask(rhs);
      decoder_v2.set_mask(rhs);
    } else if (operation == "mem") {
      string address;
      getline(lhsstream, address, ']');
      decoder.update_mem(stoll(address), DockerDecoder::lltobit(rhs));
      decoder_v2.update_mem_v2(DockerDecoder::lltobit(address),
                               DockerDecoder::lltobit(rhs));
    }
  }
  cout << "Part 1: " << decoder.mem_sum() << endl;
  cout << "Part 2: " << decoder_v2.mem_sum_v2() << endl;
}
