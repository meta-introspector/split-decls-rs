macro_rules! deps {
    () => {
        FileAstId!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl < N > Copy for FileAstId < N > { }
    };
}

impl_16!()