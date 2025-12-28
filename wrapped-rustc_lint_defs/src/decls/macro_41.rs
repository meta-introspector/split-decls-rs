macro_rules! macro_41 {
    () => {
        declare_lint ! { # [doc = " The `deprecated` lint detects use of deprecated items."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " #[deprecated]"] # [doc = " fn foo() {}"] # [doc = ""] # [doc = " fn bar() {"] # [doc = "     foo();"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " Items may be marked \"deprecated\" with the [`deprecated` attribute] to"] # [doc = " indicate that they should no longer be used. Usually the attribute"] # [doc = " should include a note on what to use instead, or check the"] # [doc = " documentation."] # [doc = ""] # [doc = " [`deprecated` attribute]: https://doc.rust-lang.org/reference/attributes/diagnostics.html#the-deprecated-attribute"] pub DEPRECATED , Warn , "detects use of deprecated items" , report_in_external_macro }
    };
}

macro_41!()