#include <vector>
void duplicate_vector(std::vector<uint32_t> v){
    for (auto&n: v){
        v.push_back(n);
    }
}

int main(){

    std::vector<uint32_t> v {1,2,3,4,5,6,7,8,9,10,11};
    duplicate_vector(v);
    return 0;
}