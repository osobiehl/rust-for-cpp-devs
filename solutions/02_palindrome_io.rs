// Function to check if a string is a palindrome
fn is_palindrome(input: &str) -> bool {
    let cleaned: String = input
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect();

    cleaned == cleaned.chars().rev().collect::<String>()
}

fn main() {
    println!("Enter a string:");
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    // Trim the input to remove any trailing newline
    let input = input.trim();

    if is_palindrome(input) {
        println!("The input is a palindrome.");
    } else {
        println!("The input is not a palindrome.");
    }
}