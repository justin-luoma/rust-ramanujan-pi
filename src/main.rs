use std::ops::Mul;

use rug::Float;
use rug::ops::Pow;

const PREC: u32 = 20000;

fn main() {
    let pi = Float::with_val(PREC, 0).acos().mul(2);
    let rpi = ramanujan_pi_calc();

    println!("{:.17}", pi);
    println!("{:.17}", rpi);
}

fn ramanujan_pi_calc() -> Float {
    let mut sum: Float = Float::with_val(PREC, 0);

    for n in 0..5000u32 {
        let part_1_a: Float = Float::with_val(PREC, Float::factorial(4 * n));
        let part_1_b = Float::with_val(PREC, Float::factorial(n)).pow(4);

        let part_1 = part_1_a / part_1_b;

        let part_2_a= Float::with_val(PREC, 26_390 * n + 1_103);
        let part_2_b = Float::with_val(PREC, 396).pow(4 * n);

        let part_2 = part_2_a / part_2_b;

        sum += part_1 * part_2;
    }
    let i = Float::with_val(PREC, 8).sqrt()
        / Float::with_val(PREC, 99).pow(2);

    Float::with_val(PREC, 1) / (sum * i)
}
