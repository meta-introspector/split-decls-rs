macro_rules! deps {
    () => {
        FutureIncompatibleInfo!();
        FutureIncompatibilityReason!();
    };
}

macro_rules! macro_69 {
    () => {
        deps!();
        declare_lint ! { # [doc = " The `const_evaluatable_unchecked` lint detects a generic constant used"] # [doc = " in a type."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " const fn foo<T>() -> usize {"] # [doc = "     if size_of::<*mut T>() < 8 { // size of *mut T does not depend on T"] # [doc = "         4"] # [doc = "     } else {"] # [doc = "         8"] # [doc = "     }"] # [doc = " }"] # [doc = ""] # [doc = " fn test<T>() {"] # [doc = "     let _ = [0; foo::<T>()];"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " In the 1.43 release, some uses of generic parameters in array repeat"] # [doc = " expressions were accidentally allowed. This is a [future-incompatible]"] # [doc = " lint to transition this to a hard error in the future. See [issue"] # [doc = " #76200] for a more detailed description and possible fixes."] # [doc = ""] # [doc = " [future-incompatible]: ../index.md#future-incompatible-lints"] # [doc = " [issue #76200]: https://github.com/rust-lang/rust/issues/76200"] pub CONST_EVALUATABLE_UNCHECKED , Warn , "detects a generic constant is used in a type without a emitting a warning" , @ future_incompatible = FutureIncompatibleInfo { reason : FutureIncompatibilityReason :: FutureReleaseError , reference : "issue #76200 <https://github.com/rust-lang/rust/issues/76200>" , } ; }
    };
}

macro_69!()