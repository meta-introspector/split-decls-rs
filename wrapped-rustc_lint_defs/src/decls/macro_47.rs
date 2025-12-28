macro_rules! macro_47 {
    () => {
        declare_lint ! { # [doc = " The `unused_lifetimes` lint detects lifetime parameters that are never"] # [doc = " used."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust,compile_fail"] # [doc = " #[deny(unused_lifetimes)]"] # [doc = ""] # [doc = " pub fn foo<'a>() {}"] # [doc = " ```"] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " Unused lifetime parameters may signal a mistake or unfinished code."] # [doc = " Consider removing the parameter."] pub UNUSED_LIFETIMES , Allow , "detects lifetime parameters that are never used" }
    };
}

macro_47!();