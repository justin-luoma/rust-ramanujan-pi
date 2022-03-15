use pi::ramanujan::ramanujan_pi_calc;

use clap::Parser;

#[derive(Parser)]
#[clap(name = "PI Calculator")]
struct Args {
    /// Sets the number of iterations
    #[clap(short, long, default_value_t = 100)]
    iterations: u32,
}

fn main() {
    let args = Args::parse();

    let rpi = ramanujan_pi_calc(args.iterations);

    println!("{:.42}", rpi);
}
