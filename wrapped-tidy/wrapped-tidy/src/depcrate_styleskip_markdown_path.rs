// Generated macro for skip_markdown_path (function)
macro_rules! Depcrate_styleskip_markdown_path {
() => {
// Module: crate::style
// Provides: {"skip_markdown_path"}
// Dependencies: {}
fn skip_markdown_path (path : & Path) -> bool { const SKIP_MD : & [& str] = & ["src/doc/edition-guide" , "src/doc/embedded-book" , "src/doc/nomicon" , "src/doc/reference" , "src/doc/rust-by-example" , "src/doc/rustc-dev-guide" ,] ; SKIP_MD . iter () . any (| p | path . ends_with (p)) }
};
}
