fn main() {
    let reference_to_nothing = dangle();

    println!("{}", reference_to_nothing);
}

fn dangle() -> String {
    let s = String::from("hello");

    // Returning a reference won't work, as the String data it is referencing is dropped once the
    // scope of dangle() ends
    // &s

    // We need to return the string itself
    s
}
