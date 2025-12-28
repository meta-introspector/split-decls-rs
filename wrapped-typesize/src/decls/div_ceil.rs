macro_rules! div_ceil {
    () => {
        # [inline] pub const fn div_ceil (n : usize , rhs : usize) -> usize { let d = n / rhs ; let r = n % rhs ; if r > 0 { d + 1 } else { d } }
    };
}

div_ceil!();