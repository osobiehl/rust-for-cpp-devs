
fn main(){
    let mut my_array: [i32; 10] = [0;10];
    for i in 0..3{
        my_array[i] =  i + 1;
    }
    let my_slice:  &[i32] = &my_array[..3];
    println!("{}", my_slice);

}
