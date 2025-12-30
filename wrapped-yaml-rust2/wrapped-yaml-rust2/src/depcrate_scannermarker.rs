// Generated macro for Marker (struct)
macro_rules! Depcrate_scannerMarker {
() => {
// Module: crate::scanner
// Provides: {"Marker"}
// Dependencies: {}
# [doc = " A location in a yaml document."] # [derive (Clone , Copy , PartialEq , Debug , Eq)] pub struct Marker { # [doc = " The index (in chars) in the input string."] index : usize , # [doc = " The line (1-indexed)."] line : usize , # [doc = " The column (1-indexed)."] col : usize , }
};
}
