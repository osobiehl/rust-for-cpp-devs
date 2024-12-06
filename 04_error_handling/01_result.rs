use std::fs::File;
use std::io::Read;

// this code compiles but panics, how can we change it so the error is recoverable?
fn main() {
    let file: Result<File, std::io::Error> = File::open("diary.txt");
    let mut file = file.unwrap();
    let mut contents = String::new();
    let bytes = file.read_to_string(&mut contents).unwrap();
    println!("Dear diary: {contents} ({bytes} bytes)");

}