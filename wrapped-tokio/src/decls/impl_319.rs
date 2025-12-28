macro_rules! deps {
    () => {
        RngSeed!();
    };
}

macro_rules! impl_319 {
    () => {
        deps!();
        impl RngSeed { # [doc = " Creates a random seed using loom internally."] pub (crate) fn new () -> Self { Self :: from_u64 (crate :: loom :: rand :: seed ()) } fn from_u64 (seed : u64) -> Self { let one = (seed >> 32) as u32 ; let mut two = seed as u32 ; if two == 0 { two = 1 ; } Self :: from_pair (one , two) } fn from_pair (s : u32 , r : u32) -> Self { Self { s , r } } }
    };
}

impl_319!();