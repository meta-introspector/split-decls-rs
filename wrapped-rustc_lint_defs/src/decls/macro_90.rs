macro_rules! macro_90 {
    () => {
        declare_lint ! { # [doc = " The `missing_abi` lint detects cases where the ABI is omitted from"] # [doc = " `extern` declarations."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust,compile_fail"] # [doc = " #![deny(missing_abi)]"] # [doc = ""] # [doc = " extern fn foo() {}"] # [doc = " ```"] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " For historic reasons, Rust implicitly selects `C` as the default ABI for"] # [doc = " `extern` declarations. [Other ABIs] like `C-unwind` and `system` have"] # [doc = " been added since then, and especially with their addition seeing the ABI"] # [doc = " easily makes code review easier."] # [doc = ""] # [doc = " [Other ABIs]: https://doc.rust-lang.org/reference/items/external-blocks.html#abi"] pub MISSING_ABI , Warn , "No declared ABI for extern declaration" }
    };
}

macro_90!();