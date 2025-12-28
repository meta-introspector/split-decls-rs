macro_rules! macro_73 {
    () => {
        declare_lint ! { # [doc = " The `useless_deprecated` lint detects deprecation attributes with no effect."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust,compile_fail"] # [doc = " struct X;"] # [doc = ""] # [doc = " #[deprecated = \"message\"]"] # [doc = " impl Default for X {"] # [doc = "     fn default() -> Self {"] # [doc = "         X"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " Deprecation attributes have no effect on trait implementations."] pub USELESS_DEPRECATED , Deny , "detects deprecation attributes with no effect" , }
    };
}

macro_73!();