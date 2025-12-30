// Generated macro for ScanError (struct)
macro_rules! Depcrate_scannerScanError {
() => {
// Module: crate::scanner
// Provides: {"ScanError"}
// Dependencies: {}
# [doc = " An error that occurred while scanning."] # [derive (Clone , PartialEq , Debug , Eq)] pub struct ScanError { # [doc = " The position at which the error happened in the source."] mark : Marker , # [doc = " Human-readable details about the error."] info : String , }
};
}
