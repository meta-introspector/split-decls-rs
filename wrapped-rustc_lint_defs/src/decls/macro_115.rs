macro_rules! macro_115 {
    () => {
        declare_lint ! { # [doc = " The `misplaced_diagnostic_attributes` lint detects wrongly placed diagnostic attributes."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " #[diagnostic::do_not_recommend]"] # [doc = " struct NotUserFacing;"] # [doc = " ```"] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " It is usually a mistake to specify a diagnostic attribute on an item it is not meant for."] # [doc = " For example, `#[diagnostic::do_not_recommend]` can only be placed on trait implementations,"] # [doc = " and does nothing if placed elsewhere. See the [reference] for a list of diagnostic"] # [doc = " attributes and their correct positions."] # [doc = ""] # [doc = " [reference]: https://doc.rust-lang.org/nightly/reference/attributes/diagnostics.html#the-diagnostic-tool-attribute-namespace"] pub MISPLACED_DIAGNOSTIC_ATTRIBUTES , Warn , "detects diagnostic attributes that are placed on the wrong item" , }
    };
}

macro_115!()