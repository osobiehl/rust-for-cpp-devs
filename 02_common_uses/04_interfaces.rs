trait Drawable{
    fn draw(&self);
}

struct Circle {

}

struct Square{

}

impl Drawable for Circle {
    fn draw(&self){
        println!("circle");
    }
}

impl Drawable for Square {
    fn draw(&self){
        println!("square");
    }
}

fn runtime_polymorphism(drawable: &Drawable){
    drawable.draw();
}

fn compile_time_polymorphism<T: Drawable>( drawable: &T){
    drawable.draw();
}



fn main(){
    let c = Circle{};
    runtime_polymorphism(c);
    compile_time_polymorphism(c);

}
