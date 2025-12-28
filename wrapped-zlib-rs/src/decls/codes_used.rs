macro_rules! deps {
    () => {
        InflateStream!();
    };
}

macro_rules! codes_used {
    () => {
        deps!();
        pub fn codes_used (stream : & InflateStream) -> usize { stream . state . next }
    };
}

codes_used!();