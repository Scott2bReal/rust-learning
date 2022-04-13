fn main() {
    let condition = true;
    let number = if condition { 5 } else { "six" }; // mismatched types error

    println!("the value of number is {}", number);
}
