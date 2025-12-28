macro_rules! div_ceil {
    () => {
        fn div_ceil (lhs : usize , rhs : usize) -> usize { let d = lhs / rhs ; let r = lhs % rhs ; if r > 0 { d + 1 } else { d } }
    };
}

div_ceil!();