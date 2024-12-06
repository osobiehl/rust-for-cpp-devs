fn main() {
    const MAX_NUMBER: u32 = 100;

    for i in 1..=MAX_NUMBER {
        let result = match (i % 3, i % 5) {
            (0, 0) => "FizzBuzz".to_string(),
            (0, _) => "Fizz".to_string(),
            (_, 0) => "Buzz".to_string(),
            _ => i.to_string(),
        };

        println!("{}", result);
    }
}