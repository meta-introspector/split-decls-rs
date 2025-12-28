macro_rules! deps {
    () => {
        FutureIncompatibilityReason!();
        FutureIncompatibleInfo!();
    };
}

macro_rules! macro_40 {
    () => {
        deps!();
        declare_lint ! { # [doc = " The `coherence_leak_check` lint detects conflicting implementations of"] # [doc = " a trait that are only distinguished by the old leak-check code."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " trait SomeTrait { }"] # [doc = " impl SomeTrait for for<'a> fn(&'a u8) { }"] # [doc = " impl<'a> SomeTrait for fn(&'a u8) { }"] # [doc = " ```"] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " In the past, the compiler would accept trait implementations for"] # [doc = " identical functions that differed only in where the lifetime binder"] # [doc = " appeared. Due to a change in the borrow checker implementation to fix"] # [doc = " several bugs, this is no longer allowed. However, since this affects"] # [doc = " existing code, this is a [future-incompatible] lint to transition this"] # [doc = " to a hard error in the future."] # [doc = ""] # [doc = " Code relying on this pattern should introduce \"[newtypes]\","] # [doc = " like `struct Foo(for<'a> fn(&'a u8))`."] # [doc = ""] # [doc = " See [issue #56105] for more details."] # [doc = ""] # [doc = " [issue #56105]: https://github.com/rust-lang/rust/issues/56105"] # [doc = " [newtypes]: https://doc.rust-lang.org/book/ch19-04-advanced-types.html#using-the-newtype-pattern-for-type-safety-and-abstraction"] # [doc = " [future-incompatible]: ../index.md#future-incompatible-lints"] pub COHERENCE_LEAK_CHECK , Warn , "distinct impls distinguished only by the leak-check code" , @ future_incompatible = FutureIncompatibleInfo { reason : FutureIncompatibilityReason :: Custom ("the behavior may change in a future release") , reference : "issue #56105 <https://github.com/rust-lang/rust/issues/56105>" , } ; }
    };
}

macro_40!();