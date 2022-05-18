fn main() {
    let s1 = String::from("Hello world");
    let word1 = first_word(&s1);
    let word2 = second_word(&s1);

    println!("First word: {}, Second word: {}", word1, word2);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item ) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn second_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[i..];
        }
    }

    &s[..]
}
