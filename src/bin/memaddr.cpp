#include <iostream>
#include <utility>
#include <vector>
using namespace std;

class Addr {
  private:
    static pair<vector<int>, vector<int>> realizer(vector<int> arr,
                                                   vector<int> index) {
        if ((arr.size() % 2 != 0) || (arr.size() / 2 != index.size())) {
            return {};
        }
        vector<int> ra = {};
        vector<int> ri = {};
        for (int i = 0; i < arr.size() / 2; ++i) {
            int curr = i * 2;
            int low = arr[curr];
            int up = arr[curr + 1];
            if (low > up) {
                cout << "Invalid indices" << endl;
                return {};
            }
            ra.push_back(up - low + 1);
            ri.push_back(index[i] - low);
        }
        return {ra, ri};
    }

  public:
    static int rmo(vector<int> arr, vector<int> &index) {
        pair<vector<int>, vector<int>> ai = realizer(arr, index);
        if (ai.first.empty() || ai.second.empty()) {
            return {};
        };
        if (ai.second[0] < 0 || ai.first[0] <= ai.second[0]) {
            return {};
        }
        int offset = ai.second[0];
        for (int i = 1; i < ai.second.size(); ++i) {
            if (ai.second[i] < 0 || ai.first[i] <= ai.second[i]) {
                return {};
            }
            offset = offset * ai.first[i] + ai.second[i];
        }
        return offset;
    }

    static int cmo(vector<int> arr, vector<int> &index) {
        pair<vector<int>, vector<int>> ai = realizer(arr, index);
        if (ai.first.empty() || ai.second.empty()) {
            return {};
        };

        if (ai.second[ai.second.size() - 1] < 0 ||
            ai.first[ai.second.size() - 1] <= ai.second[ai.second.size() - 1]) {
            return {};
        }
        int offset = ai.second[ai.second.size() - 1];
        for (int i = ai.second.size() - 2; i >= 0; --i) {
            if (ai.second[i] < 0 || ai.first[i] <= ai.second[i]) {
                return {};
            }
            offset = offset * ai.first[i] + ai.second[i];
        }
        return offset;
    }
};

int main() {
    vector<int> v = {-5, 5, 7, 20};
    vector<int> i = {3, 10};
    cout << Addr::rmo(v, i) << endl;
    cout << Addr::cmo(v, i) << endl;
}
