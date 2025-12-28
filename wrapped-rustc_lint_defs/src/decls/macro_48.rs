macro_rules! macro_48 {
    () => {
        declare_lint ! { # [doc = " The `redundant_lifetimes` lint detects lifetime parameters that are"] # [doc = " redundant because they are equal to another named lifetime."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust,compile_fail"] # [doc = " #[deny(redundant_lifetimes)]"] # [doc = ""] # [doc = " // `'a = 'static`, so all usages of `'a` can be replaced with `'static`"] # [doc = " pub fn bar<'a: 'static>() {}"] # [doc = ""] # [doc = " // `'a = 'b`, so all usages of `'b` can be replaced with `'a`"] # [doc = " pub fn bar<'a: 'b, 'b: 'a>() {}"] # [doc = " ```"] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " Unused lifetime parameters may signal a mistake or unfinished code."] # [doc = " Consider removing the parameter."] pub REDUNDANT_LIFETIMES , Allow , "detects lifetime parameters that are redundant because they are equal to some other named lifetime" }
    };
}

macro_48!();