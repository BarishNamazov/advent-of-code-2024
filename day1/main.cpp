#include <algorithm>
#include <iostream>
#include <unordered_map>
#include <vector>

using namespace std;

struct Input {
  vector<int> a, b;

  size_t size() const { return a.size(); }
};

void part1(Input input) {
  vector<int> &a = input.a;
  vector<int> &b = input.b;

  sort(a.begin(), a.end());
  sort(b.begin(), b.end());
  long long res = 0;
  for (int i = 0; i < input.size(); i++) {
    res += abs(a[i] - b[i]);
  }
  cout << res << endl;
}

void part2(Input input) {
  std::unordered_map<int, int> freq;
  for (const auto &x : input.b) {
    freq[x]++;
  }

  long long similarity = 0;
  for (const auto &x : input.a) {
    similarity += 1ll * x * freq[x];
  }

  cout << similarity << endl;
}

int main() {
  freopen("input.txt", "r", stdin);
  Input input;
  for (int a, b; cin >> a >> b;) {
    input.a.push_back(a);
    input.b.push_back(b);
  }
  part1(input);
  part2(input);
  return 0;
}
