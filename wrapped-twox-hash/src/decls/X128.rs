macro_rules! X128 {
    () => {
        # [doc = " THis exists just to easily map the XXH3 algorithm to Rust as the"] # [doc = " algorithm describes 128-bit results as a pair of high and low u64"] # [doc = " values."] # [derive (Copy , Clone)] pub (crate) struct X128 { pub low : u64 , pub high : u64 , }
    };
}

X128!();