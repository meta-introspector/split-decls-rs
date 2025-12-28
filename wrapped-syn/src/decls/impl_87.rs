macro_rules! deps {
    () => {
        BigInt!();
    };
}

macro_rules! impl_87 {
    () => {
        deps!();
        impl AddAssign < u8 > for BigInt { fn add_assign (& mut self , mut increment : u8) { self . reserve_two_digits () ; let mut i = 0 ; while increment > 0 { let sum = self . digits [i] + increment ; self . digits [i] = sum % 10 ; increment = sum / 10 ; i += 1 ; } } }
    };
}

impl_87!();