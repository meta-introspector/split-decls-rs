macro_rules! deps {
    () => {
        Subtree!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl < S > Subtree < S > { pub fn usize_len (& self) -> usize { self . len as usize } }
    };
}

impl_16!()