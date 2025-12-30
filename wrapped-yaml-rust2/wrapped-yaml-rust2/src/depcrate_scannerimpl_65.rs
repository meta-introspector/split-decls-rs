// Generated macro for impl_65 (impl)
macro_rules! Depcrate_scannerimpl_65 {
() => {
// Module: crate::scanner
// Provides: {"impl_65"}
// Dependencies: {}
impl ScanError { # [doc = " Create a new error from a location and an error string."] # [must_use] pub fn new (loc : Marker , info : & str) -> ScanError { ScanError { mark : loc , info : info . to_owned () , } } # [doc = " Create a new error from a location and an error string."] # [must_use] pub fn new_string (loc : Marker , info : String) -> ScanError { ScanError { mark : loc , info } } # [doc = " Return the marker pointing to the error in the source."] # [must_use] pub fn marker (& self) -> & Marker { & self . mark } # [doc = " Return the information string describing the error that happened."] # [must_use] pub fn info (& self) -> & str { self . info . as_ref () } }
};
}
