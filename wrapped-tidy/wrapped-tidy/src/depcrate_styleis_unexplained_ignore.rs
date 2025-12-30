// Generated macro for is_unexplained_ignore (function)
macro_rules! Depcrate_styleis_unexplained_ignore {
() => {
// Module: crate::style
// Provides: {"is_unexplained_ignore"}
// Dependencies: {}
fn is_unexplained_ignore (extension : & str , line : & str) -> bool { if ! line . ends_with ("```ignore") && ! line . ends_with ("```rust,ignore") { return false ; } if extension == "md" && line . trim () . starts_with ("//") { return false ; } true }
};
}
