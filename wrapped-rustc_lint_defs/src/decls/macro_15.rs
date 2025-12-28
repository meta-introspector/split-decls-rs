macro_rules! macro_15 {
    () => {
        declare_lint ! { # [doc = " The `unfulfilled_lint_expectations` lint detects when a lint expectation is"] # [doc = " unfulfilled."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " #[expect(unused_variables)]"] # [doc = " let x = 10;"] # [doc = " println!(\"{}\", x);"] # [doc = " ```"] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " The `#[expect]` attribute can be used to create a lint expectation. The"] # [doc = " expectation is fulfilled, if a `#[warn]` attribute at the same location"] # [doc = " would result in a lint emission. If the expectation is unfulfilled,"] # [doc = " because no lint was emitted, this lint will be emitted on the attribute."] # [doc = ""] pub UNFULFILLED_LINT_EXPECTATIONS , Warn , "unfulfilled lint expectation" }
    };
}

macro_15!();