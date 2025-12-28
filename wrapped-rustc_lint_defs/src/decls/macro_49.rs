macro_rules! deps {
    () => {
        FutureIncompatibilityReason!();
        FutureIncompatibleInfo!();
    };
}

macro_rules! macro_49 {
    () => {
        deps!();
        declare_lint ! { # [doc = " The `tyvar_behind_raw_pointer` lint detects raw pointer to an"] # [doc = " inference variable."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust,edition2015"] # [doc = " // edition 2015"] # [doc = " let data = std::ptr::null();"] # [doc = " let _ = &data as *const *const ();"] # [doc = ""] # [doc = " if data.is_null() {}"] # [doc = " ```"] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " This kind of inference was previously allowed, but with the future"] # [doc = " arrival of [arbitrary self types], this can introduce ambiguity. To"] # [doc = " resolve this, use an explicit type instead of relying on type"] # [doc = " inference."] # [doc = ""] # [doc = " This is a [future-incompatible] lint to transition this to a hard"] # [doc = " error in the 2018 edition. See [issue #46906] for more details. This"] # [doc = " is currently a hard-error on the 2018 edition, and is \"warn\" by"] # [doc = " default in the 2015 edition."] # [doc = ""] # [doc = " [arbitrary self types]: https://github.com/rust-lang/rust/issues/44874"] # [doc = " [issue #46906]: https://github.com/rust-lang/rust/issues/46906"] # [doc = " [future-incompatible]: ../index.md#future-incompatible-lints"] pub TYVAR_BEHIND_RAW_POINTER , Warn , "raw pointer to an inference variable" , @ future_incompatible = FutureIncompatibleInfo { reason : FutureIncompatibilityReason :: EditionError (Edition :: Edition2018) , reference : "issue #46906 <https://github.com/rust-lang/rust/issues/46906>" , } ; }
    };
}

macro_49!()