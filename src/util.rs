use rug::Float;
use crate::PREC;

pub fn factorial(n: u32) -> Float {
    Float::with_val(PREC, Float::factorial(n))
}

pub fn float(n: u32) -> Float {
    Float::with_val(PREC, n)
}
