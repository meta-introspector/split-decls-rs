macro_rules! Rust2024IncompatiblePatSugg {
    () => {
        pub (crate) struct Rust2024IncompatiblePatSugg { # [doc = " If true, our suggestion is to elide explicit binding modifiers."] # [doc = " If false, our suggestion is to make the pattern fully explicit."] pub (crate) suggest_eliding_modes : bool , pub (crate) suggestion : Vec < (Span , String) > , pub (crate) ref_pattern_count : usize , pub (crate) binding_mode_count : usize , # [doc = " Labels for where incompatibility-causing by-ref default binding modes were introduced."] pub (crate) default_mode_labels : FxIndexMap < Span , ty :: Mutability > , }
    };
}

Rust2024IncompatiblePatSugg!();