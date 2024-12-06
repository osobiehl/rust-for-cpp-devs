#include <iostream>
#include <string>
#include <vector>

int main() {
    constexpr int maxNumber = 100;

    for (int i = 1; i <= maxNumber; ++i) {
        std::string result;

        if (i % 3 == 0) result += "Fizz";
        if (i % 5 == 0) result += "Buzz";

        std::cout << (result.empty() ? std::to_string(i) : result) << '\n';
    }

    return 0;
}