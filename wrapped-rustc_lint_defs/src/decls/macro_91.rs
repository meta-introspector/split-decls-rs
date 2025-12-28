macro_rules! macro_91 {
    () => {
        declare_lint ! { # [doc = " The `invalid_doc_attributes` lint detects when the `#[doc(...)]` is"] # [doc = " misused."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust,compile_fail"] # [doc = " #![deny(warnings)]"] # [doc = ""] # [doc = " pub mod submodule {"] # [doc = "     #![doc(test(no_crate_inject))]"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " Previously, incorrect usage of the `#[doc(..)]` attribute was not"] # [doc = " being validated. Usually these should be rejected as a hard error,"] # [doc = " but this lint was introduced to avoid breaking any existing"] # [doc = " crates which included them."] pub INVALID_DOC_ATTRIBUTES , Deny , "detects invalid `#[doc(...)]` attributes" , }
    };
}

macro_91!();