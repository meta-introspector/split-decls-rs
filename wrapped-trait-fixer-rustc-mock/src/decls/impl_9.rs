macro_rules! deps {
    () => {
        Symbol!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl Symbol { pub fn intern (_s : & str) -> Self { Symbol } }
    };
}

impl_9!();