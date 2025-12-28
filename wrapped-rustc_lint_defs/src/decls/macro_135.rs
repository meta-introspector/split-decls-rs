macro_rules! deps {
    () => {
        FutureIncompatibleInfo!();
        FutureIncompatibilityReason!();
    };
}

macro_rules! macro_135 {
    () => {
        deps!();
        declare_lint ! { # [doc = " The `macro_extended_temporary_scopes` lint detects borrowed temporary"] # [doc = " values in arguments to `pin!` and formatting macros which have longer"] # [doc = " lifetimes than intended due to a bug in the compiler. For more"] # [doc = " information on temporary scopes and lifetime extension, see the"] # [doc = " [Rust Reference]."] # [doc = ""] # [doc = " [Rust Reference]: https://doc.rust-lang.org/reference/destructors.html#temporary-scopes"] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " # fn cond() -> bool { true }"] # [doc = " # fn build_string() -> String { String::new() }"] # [doc = " fn main() {"] # [doc = "     println!(\"{:?}{}\", (), if cond() { &build_string() } else { \"\" });"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Recommended fix"] # [doc = ""] # [doc = " To extend the lifetimes of temporaries borrowed in macro arguments,"] # [doc = " create separate definitions for them with `let` statements."] # [doc = ""] # [doc = " ```rust"] # [doc = " # fn cond() -> bool { true }"] # [doc = " # fn build_string() -> String { String::new() }"] # [doc = " fn main() {"] # [doc = "     let string = if cond() { &build_string() } else { \"\" };"] # [doc = "     println!(\"{:?}{}\", (), string);"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " Due to a compiler bug, `pin!` and formatting macros were able to extend"] # [doc = " the lifetimes of temporaries borrowed in their arguments past their"] # [doc = " usual scopes. The bug is fixed in future Rust versions, so we issue this"] # [doc = " future-incompatibility warning for code that may stop compiling or may"] # [doc = " change in behavior thereafter."] pub MACRO_EXTENDED_TEMPORARY_SCOPES , Warn , "detects when a lifetime-extended temporary borrowed in a macro argument has a future-incompatible scope." , @ future_incompatible = FutureIncompatibleInfo { reason : FutureIncompatibilityReason :: FutureReleaseError , reference : "<https://doc.rust-lang.org/rustc/lints/listing/warn-by-default.html#macro-extended-temporary-scopes>" , } ; }
    };
}

macro_135!();