const CONST_NUMBER: i32 = 10;

fn mutate(x: &mut i32) {
    *x  = CONST_NUMBER;
}

fn main() {
    int x = 0; 

    println('x is: ', x);

    x = 8;

    println!("x is: {x}")
    mutate(x);

    println!("{x:?}");
    

}