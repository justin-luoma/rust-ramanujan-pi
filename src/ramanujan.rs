use crate::util::{factorial, float};
use crate::PREC;
use rug::ops::Pow;
use rug::Float;

pub fn ramanujan_pi_calc(it: u32) -> Float {
    let mut sum: Float = Float::with_val(PREC, 0);

    for n in 0..it {
        let part_1_a = factorial(4 * n);
        let part_1_b = factorial(n).pow(4);

        let part_1 = part_1_a / part_1_b;

        let part_2_a = float(26_390 * n + 1_103);
        let part_2_b = float(396).pow(4 * n);

        let part_2 = part_2_a / part_2_b;

        sum += part_1 * part_2;
    }
    let i = float(8).sqrt() / float(99).pow(2);

    float(1) / (sum * i)
}
