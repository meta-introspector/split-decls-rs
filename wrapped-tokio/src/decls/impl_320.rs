macro_rules! deps {
    () => {
        FastRand!();
        RngSeed!();
    };
}

macro_rules! impl_320 {
    () => {
        deps!();
        impl FastRand { # [doc = " Initialize a new fast random number generator using the default source of entropy."] pub (crate) fn new () -> FastRand { FastRand :: from_seed (RngSeed :: new ()) } # [doc = " Initializes a new, thread-local, fast random number generator."] pub (crate) fn from_seed (seed : RngSeed) -> FastRand { FastRand { one : seed . s , two : seed . r , } } # [cfg (any (feature = "macros" , feature = "rt-multi-thread" , all (feature = "sync" , feature = "rt")))] pub (crate) fn fastrand_n (& mut self , n : u32) -> u32 { let mul = (self . fastrand () as u64) . wrapping_mul (n as u64) ; (mul >> 32) as u32 } fn fastrand (& mut self) -> u32 { let mut s1 = self . one ; let s0 = self . two ; s1 ^= s1 << 17 ; s1 = s1 ^ s0 ^ s1 >> 7 ^ s0 >> 16 ; self . one = s0 ; self . two = s1 ; s0 . wrapping_add (s1) } }
    };
}

impl_320!();