macro_rules! deps {
    () => {
        FutureIncompatibilityReason!();
        FutureIncompatibleInfo!();
    };
}

macro_rules! macro_44 {
    () => {
        deps!();
        declare_lint ! { # [doc = " The `rust_2024_incompatible_pat` lint"] # [doc = " detects patterns whose meaning will change in the Rust 2024 edition."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust,edition2021"] # [doc = " #![warn(rust_2024_incompatible_pat)]"] # [doc = ""] # [doc = " if let Some(&a) = &Some(&0u8) {"] # [doc = "     let _: u8 = a;"] # [doc = " }"] # [doc = " if let Some(mut _a) = &mut Some(0u8) {"] # [doc = "     _a = 7u8;"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " In Rust 2024 and above, the `mut` keyword does not reset the pattern binding mode,"] # [doc = " and nor do `&` or `&mut` patterns. The lint will suggest code that"] # [doc = " has the same meaning in all editions."] pub RUST_2024_INCOMPATIBLE_PAT , Allow , "detects patterns whose meaning will change in Rust 2024" , @ future_incompatible = FutureIncompatibleInfo { reason : FutureIncompatibilityReason :: EditionSemanticsChange (Edition :: Edition2024) , reference : "<https://doc.rust-lang.org/nightly/edition-guide/rust-2024/match-ergonomics.html>" , } ; }
    };
}

macro_44!();