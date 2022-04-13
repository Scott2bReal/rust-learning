use std::io;

fn main() {
    let mut temp = String::new();
    let mut scale = String::new();

    loop {
        println!("Please input the temperature you'd like to convert");


        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read line!");

        let temp: f32 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        loop {
            println!("Press 1 for C -> F, press 2 for F -> C");


            io::stdin()
                .read_line(&mut scale)
                .expect("Failed to read line");

            let scale = scale.trim();

            if scale == "1" {
                let temp_in_f: f32 = celsius_to_farenheit(temp);
                println!("The temp in Farenheit is: {}", temp_in_f);
                break;
            } else if scale == "2" {
                let temp_in_c: f32 = farenheit_to_celsius(temp);
                println!("The temp in Celsius is: {}", temp_in_c);
                 break;
            } else {
                continue;
            }
        };

        break;
    }
}

fn celsius_to_farenheit(temp: f32) -> f32 {
    temp * 1.8 + 32.0
}

fn farenheit_to_celsius(temp: f32) -> f32 {
    (temp - 32.0) / 1.8
}
