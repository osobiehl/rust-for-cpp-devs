#include <iostream>
class MyClass{
    public:
    void SetMyInt(int new_value){

        my_int_ = new_value;
    }

    void Bar(){

    }
    int public_int_;
    MyClass(int my_int, int public_int): my_int_ {my_int}, public_int_ {} {}

    friend std::ostream& operator<<(std::ostream& os, const MyClass& my_class){
        os << "MyClass(my_int_: " << my_class.my_int_ << ", public_int_: " << my_class.public_int_ << ")";
    return os;
    }

    private:
    int my_int_;
};

int main(){
    MyClass my_class {3, 2};
    my_class.public_int_ = 22;
    std::cout<<my_class<<std::endl;
    return 0;
}