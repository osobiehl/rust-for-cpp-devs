
fn return_tuple() -> (i32, String) {
    return (0U, "implicit conversions suck");
}

fn main() {
    let (x, y) = return_tuple();
    println!("{:?}, {:?}", x, y);

}