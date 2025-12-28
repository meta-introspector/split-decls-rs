macro_rules! set {
    () => {
        fn set (n : u16 , idx : u16 , v : u16) -> u16 { let v = v << (BITS * idx) ; let mask = MASK << (BITS * idx) ; (n & ! mask) | v }
    };
}

set!();