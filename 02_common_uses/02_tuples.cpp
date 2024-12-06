#include <tuple>
#include <iostream>

std::tuple<int, std::string> return_tuple() {
    return {0U, "implicit conversions are great"};

}


int main(){
    int x;
    std::string y;
    std::tie(x,y) = return_tuple();
    std::cout<< "x: " << x << " y: " << y << std::endl;
}