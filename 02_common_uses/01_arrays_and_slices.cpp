#include <ranges>
#include <array>
#include <span>
#include <iostream>

void printSpan(const std::span<int>& span) {
    std::cout << "[";
    for (size_t i = 0; i < span.size(); ++i) {
        std::cout << span[i];
        if (i != span.size() - 1) {
            std::cout << ", ";
        }
    }
    std::cout << "]\n";
}

int main(){
    std::array<int, 10U> my_array {0,1,2};
    std::span<int> my_span {my_array.data(), 3U};


    printSpan(my_span);


}
