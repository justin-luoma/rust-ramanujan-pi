use pi::ramanujan::ramanujan_pi_calc;

use clap::Parser;

#[derive(Parser)]
#[clap(name = "PI Calculator")]
struct Args {
    /// Sets the number of iterations
    #[clap(short, long, default_value_t = 100)]
    iterations: u32,

    /// Sets the number of digits to display
    #[clap(short, long, default_value_t = 42)]
    digits: u32,
}

fn main() {
    let args = Args::parse();

    let pi = ramanujan_pi_calc(args.iterations);

    println!("{}", pi.to_string_radix(10, Some(args.digits as usize)));
}
