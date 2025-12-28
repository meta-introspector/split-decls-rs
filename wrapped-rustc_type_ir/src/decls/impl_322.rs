macro_rules! deps {
    () => {
        TypingMode!();
        Interner!();
    };
}

macro_rules! impl_322 {
    () => {
        deps!();
        impl < I : Interner > Eq for TypingMode < I > { }
    };
}

impl_322!();