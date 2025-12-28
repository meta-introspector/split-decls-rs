macro_rules! deps {
    () => {
        LintVec!();
    };
}

macro_rules! LintPass {
    () => {
        deps!();
        pub trait LintPass { fn name (& self) -> & 'static str ; fn get_lints (& self) -> LintVec ; }
    };
}

LintPass!()