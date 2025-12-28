macro_rules! deps {
    () => {
        FutureIncompatibleInfo!();
        FutureIncompatibilityReason!();
    };
}

macro_rules! macro_62 {
    () => {
        deps!();
        declare_lint ! { # [doc = " The `soft_unstable` lint detects unstable features that were unintentionally allowed on"] # [doc = " stable. This is a [future-incompatible] lint to transition this to a hard error in the"] # [doc = " future. See [issue #64266] for more details."] # [doc = ""] # [doc = " [issue #64266]: https://github.com/rust-lang/rust/issues/64266"] # [doc = " [future-incompatible]: ../index.md#future-incompatible-lints"] pub SOFT_UNSTABLE , Deny , "a feature gate that doesn't break dependent crates" , @ future_incompatible = FutureIncompatibleInfo { reason : FutureIncompatibilityReason :: FutureReleaseError , reference : "issue #64266 <https://github.com/rust-lang/rust/issues/64266>" , report_in_deps : true , } ; }
    };
}

macro_62!();