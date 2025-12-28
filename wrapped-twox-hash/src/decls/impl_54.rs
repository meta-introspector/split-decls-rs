macro_rules! deps {
    () => {
        State!();
    };
}

macro_rules! impl_54 {
    () => {
        deps!();
        impl State { # [doc = " Constructs the hasher with an initial seed."] pub fn with_seed (seed : u64) -> Self { Self (seed) } }
    };
}

impl_54!()