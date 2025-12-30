// Generated macro for indentation (function)
macro_rules! Depcrate_alphabeticalindentation {
() => {
// Module: crate::alphabetical
// Provides: {"indentation"}
// Dependencies: {}
fn indentation (line : & str) -> usize { line . find (| c | c != ' ') . unwrap_or (0) }
};
}
