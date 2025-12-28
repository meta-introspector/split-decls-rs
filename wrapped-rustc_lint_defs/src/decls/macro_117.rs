macro_rules! macro_117 {
    () => {
        declare_lint ! { # [doc = " The `malformed_diagnostic_format_literals` lint detects malformed diagnostic format"] # [doc = " literals."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " #[diagnostic::on_unimplemented(message = \"{Self}} does not implement `Trait`\")]"] # [doc = " trait Trait {}"] # [doc = " ```"] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " The `#[diagnostic::on_unimplemented]` attribute accepts string literal values that are"] # [doc = " similar to `format!`'s string literal. See the [reference] for details on what is permitted"] # [doc = " in this string literal."] # [doc = ""] # [doc = " [reference]: https://doc.rust-lang.org/nightly/reference/attributes/diagnostics.html#the-diagnostic-tool-attribute-namespace"] pub MALFORMED_DIAGNOSTIC_FORMAT_LITERALS , Warn , "detects diagnostic attribute with malformed diagnostic format literals" , }
    };
}

macro_117!();