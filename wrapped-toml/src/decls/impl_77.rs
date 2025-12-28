macro_rules! deps {
    () => {
        Sealed!();
    };
}

macro_rules! impl_77 {
    () => {
        deps!();
        impl Sealed for String { }
    };
}

impl_77!();