use std::collections::btree_map::Range;

fn main() {
    // Chapter III
    // ## Exercise I ##
    println!("Exercise I");

    let todays_temperature = 17.2;
    let todays_temperature_fahrenheit = to_fahrenheit(todays_temperature);

    println!(
        "Die heutige Tagestemperatur in Fahrenheut {}",
        to_fahrenheit(todays_temperature)
    );
    println!(
        "Funktionieren meine Funktionen: {}",
        (todays_temperature.round() == to_celsius(todays_temperature_fahrenheit).round()),
    );

    // ## Exercise II ##
    println!("Fibonacci Number: {}", fibonacci_number(10));

    // ## Exercise III ##
    print_chrismas_carol();
}

fn to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

fn to_fahrenheit(celsius: f64) -> f64 {
    celsius * 1.8 + 32.0
}

fn fibonacci_number(n: u32) -> u32 {
    let mut a = 0;
    let mut b = 1;
    let mut c = 0;
    let mut idx = 1;
    if n == 0 {
        return a;
    }
    if n == 1 {
        return b;
    }
    while idx < n {
        c = a + b;
        a = b;
        b = c;
        idx += 1;
    }
    return c;
}

fn print_chrismas_carol() {
    for i in 1..13 {
        print!("On the {i}th day of Christmas my true love sent to me ");
        for i in (1..i + 1).rev() {
            print!("{i}");
            if i != 1 {
                print!(" and ")
            }
        }
        println!("");
    }
}
