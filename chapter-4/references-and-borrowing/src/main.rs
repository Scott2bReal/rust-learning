fn main() {
    // This would raise an error, since change() attempts to mutate the argument
    // We need to make s mutable in order for this to work
    // let s = String::from("hello");
    let mut s = String::from("hello");

    change(&mut s);

    // Code below doesn't work! Two mutable references to a variable
    // let r1 = &mut s;
    // let r2 = &mut s;
    //
    // println!("{}, {}", r1, r2);

    // We can have multiple unmutable references to a variable
    // If we have those however, we can't also have a mutable reference
    let r1 = &s; // No problem
    let r2 = &s; // No problem
    let r3 = &mut s; // Problem!
}

fn change(some_string: &mut String) {
    some_string.push_str(", world!");
}
