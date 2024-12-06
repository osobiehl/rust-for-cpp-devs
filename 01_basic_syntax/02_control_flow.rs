fn main() {
    game_loop();
}


fn do_check_if(x: u32){
    if (x == 0) {
        println!("zero!");
    } else if (x < 100) {
        println!("biggish");
    } else {
        println!("huge");
    }
}

fn do_check_match(x: u32){

    let x_text = match x {
        0 => "zero",
        1..=100 => "biggish",
        _ => "huge"

    }

    println!("{}", x_text);


}

fn game_loop(){
    let should_be_42 = loop {
        let s = String::new()
        std::io::stdin().read_line(&mut s).expect("io failed");
        if s.starts_with("42"){
            break 42;
        }
    }
    do_check_if(should_be_42);
    do_check_match(should_be_42);

}
