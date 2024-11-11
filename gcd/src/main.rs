use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    numbers: Vec<u64>,
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);
    assert_eq!(gcd(20, 25), 5);
}

fn main() {
    let args = Args::parse();

    if args.numbers.len() < 2 {
        eprintln!("Usage: cargo run -- <numbers>");
        return;
    }

    let mut result = args.numbers[0];

    for &num in &args.numbers[1..] {
        result = gcd(result, num);
    }

    println!("GCD of {:?} is {result}", args.numbers);
}
