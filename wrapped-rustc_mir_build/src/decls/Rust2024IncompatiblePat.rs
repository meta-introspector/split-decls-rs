macro_rules! deps {
    () => {
        Rust2024IncompatiblePatSugg!();
    };
}

macro_rules! Rust2024IncompatiblePat {
    () => {
        deps!();
        # [derive (LintDiagnostic)] # [diag (mir_build_rust_2024_incompatible_pat)] pub (crate) struct Rust2024IncompatiblePat { # [subdiagnostic] pub (crate) sugg : Rust2024IncompatiblePatSugg , pub (crate) bad_modifiers : bool , pub (crate) bad_ref_pats : bool , pub (crate) is_hard_error : bool , }
    };
}

Rust2024IncompatiblePat!();