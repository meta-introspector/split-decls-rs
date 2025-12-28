macro_rules! deps {
    () => {
        Sealed!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl Sealed for dyn Error + Send + Sync + UnwindSafe + '_ { }
    };
}

impl_14!();