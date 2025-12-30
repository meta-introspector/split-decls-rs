// Generated macro for impl_230 (impl)
macro_rules! Depcrate_commentimpl_230 {
() => {
// Module: crate::comment
// Provides: {"impl_230"}
// Dependencies: {}
impl CodeBlockAttribute { # [doc = " Parse comma separated attributes list. Return rust only if all"] # [doc = " attributes are valid rust attributes"] # [doc = " See <https://doc.rust-lang.org/rustdoc/print.html#attributes>"] fn new (attributes : & str) -> CodeBlockAttribute { for attribute in attributes . split (',') { match attribute . trim () { "" | "rust" | "should_panic" | "no_run" | "edition2015" | "edition2018" | "edition2021" => () , "ignore" | "compile_fail" | "text" => return CodeBlockAttribute :: NotRust , _ => return CodeBlockAttribute :: NotRust , } } CodeBlockAttribute :: Rust } }
};
}
