// Generated macro for with_contents (function)
macro_rules! Depcratewith_contents {
() => {
// Module: crate
// Provides: {"with_contents"}
// Dependencies: {}
# [doc = " Create a new  file in a system temporary directory"] # [doc = " and writes `contents` to it."] # [doc = ""] # [doc = " # Panics"] # [doc = " Panics if it fails to create the file or fails to write all of `contents`."] # [must_use] pub fn with_contents (contents : & [u8]) -> TempFile { TempFile :: new () . unwrap () . with_contents (contents) . unwrap () }
};
}
