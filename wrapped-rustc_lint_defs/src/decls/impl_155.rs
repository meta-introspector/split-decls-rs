macro_rules! deps {
    () => {
        LintId!();
    };
}

macro_rules! impl_155 {
    () => {
        deps!();
        impl Eq for LintId { }
    };
}

impl_155!();