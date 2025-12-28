macro_rules! macro_70 {
    () => {
        declare_lint ! { # [doc = " The `function_item_references` lint detects function references that are"] # [doc = " formatted with [`fmt::Pointer`] or transmuted."] # [doc = ""] # [doc = " [`fmt::Pointer`]: https://doc.rust-lang.org/std/fmt/trait.Pointer.html"] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " fn foo() { }"] # [doc = ""] # [doc = " fn main() {"] # [doc = "     println!(\"{:p}\", &foo);"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " Taking a reference to a function may be mistaken as a way to obtain a"] # [doc = " pointer to that function. This can give unexpected results when"] # [doc = " formatting the reference as a pointer or transmuting it. This lint is"] # [doc = " issued when function references are formatted as pointers, passed as"] # [doc = " arguments bound by [`fmt::Pointer`] or transmuted."] pub FUNCTION_ITEM_REFERENCES , Warn , "suggest casting to a function pointer when attempting to take references to function items" , }
    };
}

macro_70!()