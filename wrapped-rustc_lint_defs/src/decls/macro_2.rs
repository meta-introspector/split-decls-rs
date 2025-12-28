macro_rules! deps {
    () => {
        FutureIncompatibleInfo!();
        FutureIncompatibilityReason!();
    };
}

macro_rules! macro_2 {
    () => {
        deps!();
        declare_lint ! { # [doc = " The `ill_formed_attribute_input` lint detects ill-formed attribute"] # [doc = " inputs that were previously accepted and used in practice."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust,compile_fail"] # [doc = " #[inline = \"this is not valid\"]"] # [doc = " fn foo() {}"] # [doc = " ```"] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " Previously, inputs for many built-in attributes weren't validated and"] # [doc = " nonsensical attribute inputs were accepted. After validation was"] # [doc = " added, it was determined that some existing projects made use of these"] # [doc = " invalid forms. This is a [future-incompatible] lint to transition this"] # [doc = " to a hard error in the future. See [issue #57571] for more details."] # [doc = ""] # [doc = " Check the [attribute reference] for details on the valid inputs for"] # [doc = " attributes."] # [doc = ""] # [doc = " [issue #57571]: https://github.com/rust-lang/rust/issues/57571"] # [doc = " [attribute reference]: https://doc.rust-lang.org/nightly/reference/attributes.html"] # [doc = " [future-incompatible]: ../index.md#future-incompatible-lints"] pub ILL_FORMED_ATTRIBUTE_INPUT , Deny , "ill-formed attribute inputs that were previously accepted and used in practice" , @ future_incompatible = FutureIncompatibleInfo { reason : FutureIncompatibilityReason :: FutureReleaseError , reference : "issue #57571 <https://github.com/rust-lang/rust/issues/57571>" , report_in_deps : true , } ; crate_level_only }
    };
}

macro_2!();