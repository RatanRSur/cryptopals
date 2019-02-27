fn english_score(s: &str) -> usize {
    s.chars()
        .fold(0, |acc, c| if c.is_digit(10) { acc } else { acc + 100 })
        / s.len()
}

fn find_message(s: &str) -> String {
    let chars = (0u8..16u8).map(|x| hex::encode(
}

fn main() {
    println!("{}", find_message("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736"));
}
