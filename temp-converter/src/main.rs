use clap::Parser;
use std::process;

#[derive(Parser, Debug)]
#[clap(name = "temp-converter")]
#[clap(version = "0.1.0")]
#[clap(about = "Converts Fahrenheit to Celsius and vice versa.", long_about = None)]
struct Args {
    #[clap(short, long, help = "The temperature as a unitless decimal value.")]
    temp: f64,
    #[clap(short = "-n", long, help = "The unit of measurement. Accepts 'F' or 'C' only.")]
    unit: char,
}

fn main() {
    let args = Args::parse();

    let unit: char = args.unit;
    let temp: f64 = args.temp;

    println!("celsius: {:?}", temp);
    println!("unit: {:?}", unit);

    if unit != 'F' && unit != 'C' {
        println!("The <UNIT> argument only accepts 'F' or 'C'.");
        process::exit(1);
    }

    if unit == 'F' {
        let ret_temp = (temp - 32.0) * 5.0 / 9.0;
        println!("{} in F is {:.2} in C", temp, ret_temp);
    }

    if unit == 'C' {
        let ret_temp = temp * 9.0 / 5.0 + 32.0;
        println!("{} in C is {:.2} in F", temp, ret_temp);
    }
}
