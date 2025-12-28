macro_rules! deps {
    () => {
        BigInt!();
    };
}

macro_rules! impl_88 {
    () => {
        deps!();
        impl MulAssign < u8 > for BigInt { fn mul_assign (& mut self , base : u8) { self . reserve_two_digits () ; let mut carry = 0 ; for digit in & mut self . digits { let prod = * digit * base + carry ; * digit = prod % 10 ; carry = prod / 10 ; } } }
    };
}

impl_88!()