macro_rules! deps {
    () => {
        FutureIncompatibleInfo!();
        FutureIncompatibilityReason!();
    };
}

macro_rules! macro_75 {
    () => {
        deps!();
        declare_lint ! { # [doc = " The `self_constructor_from_outer_item` lint detects cases where the `Self` constructor"] # [doc = " was silently allowed due to a bug in the resolver, and which may produce surprising"] # [doc = " and unintended behavior."] # [doc = ""] # [doc = " Using a `Self` type alias from an outer item was never intended, but was silently allowed."] # [doc = " This is deprecated -- and is a hard error when the `Self` type alias references generics"] # [doc = " that are not in scope."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust,compile_fail"] # [doc = " #![deny(self_constructor_from_outer_item)]"] # [doc = ""] # [doc = " struct S0(usize);"] # [doc = ""] # [doc = " impl S0 {"] # [doc = "     fn foo() {"] # [doc = "         const C: S0 = Self(0);"] # [doc = "         fn bar() -> S0 {"] # [doc = "             Self(0)"] # [doc = "         }"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " The `Self` type alias should not be reachable because nested items are not associated with"] # [doc = " the scope of the parameters from the parent item."] pub SELF_CONSTRUCTOR_FROM_OUTER_ITEM , Warn , "detect unsupported use of `Self` from outer item" , @ future_incompatible = FutureIncompatibleInfo { reason : FutureIncompatibilityReason :: FutureReleaseError , reference : "issue #124186 <https://github.com/rust-lang/rust/issues/124186>" , } ; }
    };
}

macro_75!();