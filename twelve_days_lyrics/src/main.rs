fn main() {
    let gifts = [
        "Twelve drummers drummin'",
        "Eleven pipers pipin'",
        "Ten lords a leapin'",
        "Nine ladies dancin'",
        "Eight maids a milkin'",
        "Seven swans a swimmin'",
        "Six geese a layin'",
        "Five golden rings",
        "Four calling birds",
        "Three french hens",
        "Two turtle doves",
        "and a partridge in a pear tree"
    ];

    let days = [
        "first",
        "second",
        "third",
        "fourth",
        "fifth",
        "sixth",
        "seventh",
        "eighth",
        "ninth",
        "tenth",
        "eleventh",
        "twelfth"
    ];

    println!("On the first day of Christmas my true love gave to me");
    println!("A partridge in a pear tree\n");

    let mut day_counter = 1;

    while day_counter < 12 {
        let mut gift_counter = 12 - day_counter;

        println!("On the {} day of Christmas my true love gave to me", 
                 days[day_counter]);

        while gift_counter < 12 {
            println!("{}", gifts[gift_counter]);
            gift_counter += 1;
        }

        println!("\n");
        day_counter += 1;
    }
}
