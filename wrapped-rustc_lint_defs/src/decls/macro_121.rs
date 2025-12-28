macro_rules! deps {
    () => {
        FutureIncompatibilityReason!();
        FutureIncompatibleInfo!();
    };
}

macro_rules! macro_121 {
    () => {
        deps!();
        declare_lint ! { # [doc = " The `elided_lifetimes_in_associated_constant` lint detects elided lifetimes"] # [doc = " in associated constants when there are other lifetimes in scope. This was"] # [doc = " accidentally supported, and this lint was later relaxed to allow eliding"] # [doc = " lifetimes to `'static` when there are no lifetimes in scope."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust,compile_fail"] # [doc = " #![deny(elided_lifetimes_in_associated_constant)]"] # [doc = ""] # [doc = " struct Foo<'a>(&'a ());"] # [doc = ""] # [doc = " impl<'a> Foo<'a> {"] # [doc = "     const STR: &str = \"hello, world\";"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " Previous version of Rust"] # [doc = ""] # [doc = " Implicit static-in-const behavior was decided [against] for associated"] # [doc = " constants because of ambiguity. This, however, regressed and the compiler"] # [doc = " erroneously treats elided lifetimes in associated constants as lifetime"] # [doc = " parameters on the impl."] # [doc = ""] # [doc = " This is a [future-incompatible] lint to transition this to a"] # [doc = " hard error in the future."] # [doc = ""] # [doc = " [against]: https://github.com/rust-lang/rust/issues/38831"] # [doc = " [future-incompatible]: ../index.md#future-incompatible-lints"] pub ELIDED_LIFETIMES_IN_ASSOCIATED_CONSTANT , Deny , "elided lifetimes cannot be used in associated constants in impls" , @ future_incompatible = FutureIncompatibleInfo { reason : FutureIncompatibilityReason :: FutureReleaseError , reference : "issue #115010 <https://github.com/rust-lang/rust/issues/115010>" , } ; }
    };
}

macro_121!()