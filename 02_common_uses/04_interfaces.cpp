#include <iostream>
#include <concepts>
class DrawableInterface {
public:
    virtual void Draw() = 0;
};


class Circle final: public DrawableInterface {
    public:
    void Draw() final {
        std::cout<<"circle"<<std::endl;
    }
};

class Square final: public DrawableInterface {
    public:
    void Draw() final {
        std::cout<<"square"<<std::endl;
    }
};

void RuntimePolymorphism(DrawableInterface& drawable){
    drawable.Draw();
}

template <typename T>
concept Drawable = requires(T& obj) {
    { obj.Draw() } -> std::same_as<void>; // Requires a draw() method that returns void
};

template <Drawable T> void CompileTimePolymorphism(T& drawable){
    drawable.Draw();
}
int main(){
    Circle c {};
    RuntimePolymorphism(c);
    CompileTimePolymorphism(c);
    return 0;
}