macro_rules! deps {
    () => {
        Subtree!();
    };
}

macro_rules! impl_61 {
    () => {
        deps!();
        impl < S > Subtree < S > { # [doc = " Count the number of tokens recursively"] pub fn count (& self) -> usize { self . usize_len () } }
    };
}

impl_61!();