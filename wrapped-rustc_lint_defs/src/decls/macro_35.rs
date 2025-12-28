macro_rules! deps {
    () => {
        FutureIncompatibleInfo!();
        FutureIncompatibilityReason!();
    };
}

macro_rules! macro_35 {
    () => {
        deps!();
        declare_lint ! { # [doc = " The `invalid_type_param_default` lint detects type parameter defaults"] # [doc = " erroneously allowed in an invalid location."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust,compile_fail"] # [doc = " fn foo<T=i32>(t: T) {}"] # [doc = " ```"] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " Default type parameters were only intended to be allowed in certain"] # [doc = " situations, but historically the compiler allowed them everywhere."] # [doc = " This is a [future-incompatible] lint to transition this to a hard"] # [doc = " error in the future. See [issue #36887] for more details."] # [doc = ""] # [doc = " [issue #36887]: https://github.com/rust-lang/rust/issues/36887"] # [doc = " [future-incompatible]: ../index.md#future-incompatible-lints"] pub INVALID_TYPE_PARAM_DEFAULT , Deny , "type parameter default erroneously allowed in invalid location" , @ future_incompatible = FutureIncompatibleInfo { reason : FutureIncompatibilityReason :: FutureReleaseError , reference : "issue #36887 <https://github.com/rust-lang/rust/issues/36887>" , report_in_deps : true , } ; }
    };
}

macro_35!();