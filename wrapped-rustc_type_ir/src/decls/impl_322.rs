macro_rules! deps {
    () => {
        Interner!();
        TypingMode!();
    };
}

macro_rules! impl_322 {
    () => {
        deps!();
        impl < I : Interner > Eq for TypingMode < I > { }
    };
}

impl_322!()