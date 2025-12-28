macro_rules! deps {
    () => {
        LintVec!();
        Lint!();
        LintPass!();
    };
}

macro_rules! impl_lint_pass {
    () => {
        deps!();
        # [doc = " Implements `LintPass for $ty` with the given list of `Lint` statics."] # [macro_export] macro_rules ! impl_lint_pass { ($ ty : ty => [$ ($ lint : expr) ,* $ (,) ?]) => { impl $ crate :: LintPass for $ ty { fn name (& self) -> &'static str { stringify ! ($ ty) } fn get_lints (& self) -> $ crate :: LintVec { vec ! [$ ($ lint) ,*] } } impl $ ty { # [allow (unused)] pub fn lint_vec () -> $ crate :: LintVec { vec ! [$ ($ lint) ,*] } } } ; }
    };
}

impl_lint_pass!()