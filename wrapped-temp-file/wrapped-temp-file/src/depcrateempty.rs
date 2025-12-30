// Generated macro for empty (function)
macro_rules! Depcrateempty {
() => {
// Module: crate
// Provides: {"empty"}
// Dependencies: {}
# [doc = " Create a new empty file in a system temporary directory."] # [doc = ""] # [doc = " # Panics"] # [doc = " Panics if it cannot create the file."] # [must_use] pub fn empty () -> TempFile { TempFile :: new () . unwrap () }
};
}
