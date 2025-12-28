macro_rules! deps {
    () => {
        LintId!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl Eq for LintId { }
    };
}

impl_19!()