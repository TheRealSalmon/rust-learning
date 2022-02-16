use clap::Parser;

// here we create the argparser using clap
#[derive(Parser, Debug)]
#[clap(name = "dyn-fib")]
#[clap(version = "0.1.0")]
#[clap(about = "returns the nth number in the fibonacci sequence", long_about = None)]
struct Args {
    #[clap(short, long, help = "n, the nth number in the sequence to obtain")]
    n_fib: u32,
}

fn main() {
    // the only argument we need is what nth value of the fibonacci sequence we'd like to obtain
    let args = Args::parse();
    let n_fib: u32 = args.n_fib;

    // we first create a vector of -1s with the size that matches the nth value we want
    let mut fib_seq: Vec<i32> = vec![-1; n_fib as usize];

    // we wrote a short function to find the correct suffix for the given nth value
    let num_suffix: &str = get_num_suffix(n_fib);
    println!(
        "the {}{} fibonacci number is: {}",
        // here we call the actual algorithm using get_nth_fib
        n_fib, num_suffix, get_nth_fib(n_fib, &mut fib_seq)
    )
}

fn get_num_suffix(n_fib: u32) -> &'static str {
    if n_fib % 10 == 1 {
        return "st"
    } else if n_fib % 10 == 2 {
        return "nd"
    } else if n_fib % 10 == 3 {
        return "rd"
    } else {
        return "th"
    }
}

fn get_nth_fib(n_fib: u32, fib_seq: &mut Vec<i32>) -> u32 {
    // first we take care of the base cases
    if n_fib == 0 {
        return 0
    }
    if n_fib == 1 {
        return 1
    }

    // then we run a for loop that will iterate through the array, updating the fibonacci values
    fib_seq[0] = 0;
    fib_seq[1] = 1;
    for i in 2..n_fib as usize {
        fib_seq[i] = fib_seq[i - 1] + fib_seq[i - 2];
    }
    // finally we return the last value in the vector
    return fib_seq[n_fib as usize - 1] as u32;
}
