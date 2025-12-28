macro_rules! deps {
    () => {
        Lint!();
        LintPass!();
    };
}

macro_rules! declare_lint_pass {
    () => {
        deps!();
        # [doc = " Declares a type named `$name` which implements `LintPass`."] # [doc = " To the right of `=>` a comma separated list of `Lint` statics is given."] # [macro_export] macro_rules ! declare_lint_pass { ($ (# [$ m : meta]) * $ name : ident => [$ ($ lint : expr) ,* $ (,) ?]) => { $ (# [$ m]) * # [derive (Copy , Clone)] pub struct $ name ; $ crate :: impl_lint_pass ! ($ name => [$ ($ lint) ,*]) ; } ; }
    };
}

declare_lint_pass!()