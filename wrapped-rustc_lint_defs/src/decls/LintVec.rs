macro_rules! deps {
    () => {
        Lint!();
    };
}

macro_rules! LintVec {
    () => {
        deps!();
        pub type LintVec = Vec < & 'static Lint > ;
    };
}

LintVec!();