macro_rules! deps {
    () => {
        FutureIncompatibilityReason!();
        FutureIncompatibleInfo!();
    };
}

macro_rules! macro_61 {
    () => {
        deps!();
        declare_lint ! { # [doc = " The `ambiguous_associated_items` lint detects ambiguity between"] # [doc = " [associated items] and [enum variants]."] # [doc = ""] # [doc = " [associated items]: https://doc.rust-lang.org/reference/items/associated-items.html"] # [doc = " [enum variants]: https://doc.rust-lang.org/reference/items/enumerations.html"] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust,compile_fail"] # [doc = " enum E {"] # [doc = "     V"] # [doc = " }"] # [doc = ""] # [doc = " trait Tr {"] # [doc = "     type V;"] # [doc = "     fn foo() -> Self::V;"] # [doc = " }"] # [doc = ""] # [doc = " impl Tr for E {"] # [doc = "     type V = u8;"] # [doc = "     // `Self::V` is ambiguous because it may refer to the associated type or"] # [doc = "     // the enum variant."] # [doc = "     fn foo() -> Self::V { 0 }"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " Previous versions of Rust did not allow accessing enum variants"] # [doc = " through [type aliases]. When this ability was added (see [RFC 2338]), this"] # [doc = " introduced some situations where it can be ambiguous what a type"] # [doc = " was referring to."] # [doc = ""] # [doc = " To fix this ambiguity, you should use a [qualified path] to explicitly"] # [doc = " state which type to use. For example, in the above example the"] # [doc = " function can be written as `fn f() -> <Self as Tr>::V { 0 }` to"] # [doc = " specifically refer to the associated type."] # [doc = ""] # [doc = " This is a [future-incompatible] lint to transition this to a hard"] # [doc = " error in the future. See [issue #57644] for more details."] # [doc = ""] # [doc = " [issue #57644]: https://github.com/rust-lang/rust/issues/57644"] # [doc = " [type aliases]: https://doc.rust-lang.org/reference/items/type-aliases.html#type-aliases"] # [doc = " [RFC 2338]: https://github.com/rust-lang/rfcs/blob/master/text/2338-type-alias-enum-variants.md"] # [doc = " [qualified path]: https://doc.rust-lang.org/reference/paths.html#qualified-paths"] # [doc = " [future-incompatible]: ../index.md#future-incompatible-lints"] pub AMBIGUOUS_ASSOCIATED_ITEMS , Deny , "ambiguous associated items" , @ future_incompatible = FutureIncompatibleInfo { reason : FutureIncompatibilityReason :: FutureReleaseError , reference : "issue #57644 <https://github.com/rust-lang/rust/issues/57644>" , } ; }
    };
}

macro_61!()