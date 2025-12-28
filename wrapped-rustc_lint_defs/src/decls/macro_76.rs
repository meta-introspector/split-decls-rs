macro_rules! deps {
    () => {
        FutureIncompatibilityReason!();
        FutureIncompatibleInfo!();
    };
}

macro_rules! macro_76 {
    () => {
        deps!();
        declare_lint ! { # [doc = " The `semicolon_in_expressions_from_macros` lint detects trailing semicolons"] # [doc = " in macro bodies when the macro is invoked in expression position."] # [doc = " This was previous accepted, but is being phased out."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust,compile_fail"] # [doc = " #![deny(semicolon_in_expressions_from_macros)]"] # [doc = " macro_rules! foo {"] # [doc = "     () => { true; }"] # [doc = " }"] # [doc = ""] # [doc = " fn main() {"] # [doc = "     let val = match true {"] # [doc = "         true => false,"] # [doc = "         _ => foo!()"] # [doc = "     };"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " Previous, Rust ignored trailing semicolon in a macro"] # [doc = " body when a macro was invoked in expression position."] # [doc = " However, this makes the treatment of semicolons in the language"] # [doc = " inconsistent, and could lead to unexpected runtime behavior"] # [doc = " in some circumstances (e.g. if the macro author expects"] # [doc = " a value to be dropped)."] # [doc = ""] # [doc = " This is a [future-incompatible] lint to transition this"] # [doc = " to a hard error in the future. See [issue #79813] for more details."] # [doc = ""] # [doc = " [issue #79813]: https://github.com/rust-lang/rust/issues/79813"] # [doc = " [future-incompatible]: ../index.md#future-incompatible-lints"] pub SEMICOLON_IN_EXPRESSIONS_FROM_MACROS , Deny , "trailing semicolon in macro body used as expression" , @ future_incompatible = FutureIncompatibleInfo { reason : FutureIncompatibilityReason :: FutureReleaseError , reference : "issue #79813 <https://github.com/rust-lang/rust/issues/79813>" , report_in_deps : true , } ; }
    };
}

macro_76!()