use crate::Bit;
use crate::Int;

pub fn main(mut a: Int, mut b: Int) -> Int {

    let mut res: Vec<Bit> = Vec::new();

    while !a.bits.is_empty() || !b.bits.is_empty() {

        let a_bit: Bit =
            match a.bits.pop() {
                Some(r) => r,
                None => Bit::Zero
            };

        let b_bit: Bit =
            match b.bits.pop() {
                Some(r) => r,
                None => Bit::Zero
            };

        let or_bit =
            match (a_bit, b_bit) {
                (Bit::Zero, Bit::Zero) => Bit::Zero,
                _ => Bit::One
            };

        res.push(or_bit)

    }

    res.reverse();

    Int {
        bits: res, sign: false
    }

}