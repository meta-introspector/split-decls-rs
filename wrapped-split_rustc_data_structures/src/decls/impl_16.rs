macro_rules! deps {
    () => {
        ToBaseN!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl ToBaseN for u64 { fn encoded_len (base : usize) -> usize { let mut max = u64 :: MAX ; let mut len = 0 ; while max > 0 { len += 1 ; max /= base as u64 ; } len } }
    };
}

impl_16!();