macro_rules! deps {
    () => {
        FutureIncompatibleInfo!();
        FutureIncompatibilityReason!();
    };
}

macro_rules! macro_131 {
    () => {
        deps!();
        declare_lint ! { # [doc = " The `rust_2024_guarded_string_incompatible_syntax` lint detects `#` tokens"] # [doc = " that will be parsed as part of a guarded string literal in Rust 2024."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust,edition2021,compile_fail"] # [doc = " #![deny(rust_2024_guarded_string_incompatible_syntax)]"] # [doc = ""] # [doc = " macro_rules! m {"] # [doc = "     (# $x:expr #) => ();"] # [doc = "     (# $x:expr) => ();"] # [doc = " }"] # [doc = ""] # [doc = " m!(#\"hey\"#);"] # [doc = " m!(#\"hello\");"] # [doc = " ```"] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " Prior to Rust 2024, `#\"hey\"#` is three tokens: the first `#`"] # [doc = " followed by the string literal `\"hey\"` then the final `#`."] # [doc = " In Rust 2024, the whole sequence is considered a single token."] # [doc = ""] # [doc = " This lint suggests to add whitespace between the leading `#`"] # [doc = " and the string to keep them separated in Rust 2024."] # [allow (rustdoc :: invalid_rust_codeblocks)] pub RUST_2024_GUARDED_STRING_INCOMPATIBLE_SYNTAX , Allow , "will be parsed as a guarded string in Rust 2024" , @ future_incompatible = FutureIncompatibleInfo { reason : FutureIncompatibilityReason :: EditionError (Edition :: Edition2024) , reference : "<https://doc.rust-lang.org/edition-guide/rust-2024/reserved-syntax.html>" , } ; crate_level_only }
    };
}

macro_131!()