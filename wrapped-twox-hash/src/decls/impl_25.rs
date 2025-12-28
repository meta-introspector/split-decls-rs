macro_rules! deps {
    () => {
        State!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        impl State { # [doc = " Constructs the hasher with an initial seed."] pub fn with_seed (seed : u32) -> Self { Self (seed) } }
    };
}

impl_25!()