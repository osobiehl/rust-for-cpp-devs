#include <iostream>
#include <string>
#include <algorithm>

// Function to check if a string is a palindrome
bool isPalindrome(const std::string& str) {
    std::string cleanedStr;
    
    // Remove non-alphanumeric characters and convert to lowercase
    std::copy_if(str.begin(), str.end(), std::back_inserter(cleanedStr), [](char c) {
        return std::isalnum(c);
    });
    std::transform(cleanedStr.begin(), cleanedStr.end(), cleanedStr.begin(), ::tolower);

    // Check if the string is equal to its reverse
    return std::equal(cleanedStr.begin(), cleanedStr.begin() + cleanedStr.size() / 2, cleanedStr.rbegin());
}

int main() {
    std::string input;

    std::cout << "Enter a string: ";
    std::getline(std::cin, input);

    if (isPalindrome(input)) {
        std::cout << "The input is a palindrome.\n";
    } else {
        std::cout << "The input is not a palindrome.\n";
    }

    return 0;
}