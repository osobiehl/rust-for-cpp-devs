#[derive(Debug)]
struct MyStruct{
    pub public_int: i32,
    my_int: i32
}


impl MyStruct{
    pub fn new(public_int: i32, my_int_new: i32) ->Self{

        return Self {
            public_int, //shorthand notation 
            my_int: my_int_new
        }
    }
    fn bar(&self) {

    }
    pub fn set_my_int(&self, new_value: i32){
        self.my_int = new_value;

    }
}


fn main(){
    let my_struct = MyStruct::new(3,2);
    my_struct.bar();
    my_struct.public_int = 22;
    my_struct.set_my_int(new_value);
    println!("{}", my_struct);

}