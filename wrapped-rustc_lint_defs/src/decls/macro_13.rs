macro_rules! macro_13 {
    () => {
        declare_lint ! { # [doc = " The `unused_qualifications` lint detects unnecessarily qualified"] # [doc = " names."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust,compile_fail"] # [doc = " #![deny(unused_qualifications)]"] # [doc = " mod foo {"] # [doc = "     pub fn bar() {}"] # [doc = " }"] # [doc = ""] # [doc = " fn main() {"] # [doc = "     use foo::bar;"] # [doc = "     foo::bar();"] # [doc = "     bar();"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " If an item from another module is already brought into scope, then"] # [doc = " there is no need to qualify it in this case. You can call `bar()`"] # [doc = " directly, without the `foo::`."] # [doc = ""] # [doc = " This lint is \"allow\" by default because it is somewhat pedantic, and"] # [doc = " doesn't indicate an actual problem, but rather a stylistic choice, and"] # [doc = " can be noisy when refactoring or moving around code."] pub UNUSED_QUALIFICATIONS , Allow , "detects unnecessarily qualified names" }
    };
}

macro_13!()