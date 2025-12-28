macro_rules! deps {
    () => {
        FutureIncompatibleInfo!();
        FutureIncompatibilityReason!();
    };
}

macro_rules! macro_122 {
    () => {
        deps!();
        declare_lint ! { # [doc = " The `private_macro_use` lint detects private macros that are imported"] # [doc = " with `#[macro_use]`."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust,ignore (needs extern crate)"] # [doc = " // extern_macro.rs"] # [doc = " macro_rules! foo_ { () => {}; }"] # [doc = " use foo_ as foo;"] # [doc = ""] # [doc = " // code.rs"] # [doc = ""] # [doc = " #![deny(private_macro_use)]"] # [doc = ""] # [doc = " #[macro_use]"] # [doc = " extern crate extern_macro;"] # [doc = ""] # [doc = " fn main() {"] # [doc = "     foo!();"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " This will produce:"] # [doc = ""] # [doc = " ```text"] # [doc = " error: cannot find macro `foo` in this scope"] # [doc = " ```"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " This lint arises from overlooking visibility checks for macros"] # [doc = " in an external crate."] # [doc = ""] # [doc = " This is a [future-incompatible] lint to transition this to a"] # [doc = " hard error in the future."] # [doc = ""] # [doc = " [future-incompatible]: ../index.md#future-incompatible-lints"] pub PRIVATE_MACRO_USE , Deny , "detects certain macro bindings that should not be re-exported" , @ future_incompatible = FutureIncompatibleInfo { reason : FutureIncompatibilityReason :: FutureReleaseError , reference : "issue #120192 <https://github.com/rust-lang/rust/issues/120192>" , report_in_deps : true , } ; }
    };
}

macro_122!()