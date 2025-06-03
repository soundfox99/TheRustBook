use std::io;

fn main() {
    print_labeled_measurement(5, 'h');

    let temperature: f32 = loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse() {
            Ok(num) => break num,
            Err(_) => {
                println!("Please enter a valid number.");    
                continue;
            }
        };
    };

    let converted_temperature = convert_fahrenheit_to_celcius(temperature);
    println!("The convert temperature is {converted_temperature}");

    let fib_num = {
        let mut sum = 0;
        let mut previous = 0;
        let mut current = 1;

        for _ in 1..45 {
            sum = current + previous;
            previous = current;
            current = sum;
        };

        sum
    };

    println!("Fibinocci number is {fib_num}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is {value}{unit_label}");
}

fn convert_fahrenheit_to_celcius(temperature: f32) -> f32 {
    (temperature - 32.0) * (5.0/9.0)
}