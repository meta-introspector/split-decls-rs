// Generated macro for find_crates (function)
macro_rules! Depcratefind_crates {
() => {
// Module: crate
// Provides: {"find_crates"}
// Dependencies: {}
# [doc = " Try to find a crate or crates if multiple crates exist from given name."] pub fn find_crates (name : & str) -> Vec < Crate > { with (| cx | cx . find_crates (name)) }
};
}
