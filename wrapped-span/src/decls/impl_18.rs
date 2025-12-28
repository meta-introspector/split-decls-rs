macro_rules! deps {
    () => {
        FileAstId!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        impl < N > Eq for FileAstId < N > { }
    };
}

impl_18!();