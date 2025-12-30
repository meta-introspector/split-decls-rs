// Generated macro for LoadError (enum)
macro_rules! Depcrate_yamlLoadError {
() => {
// Module: crate::yaml
// Provides: {"LoadError"}
// Dependencies: {}
# [doc = " An error that happened when loading a YAML document."] # [derive (Debug)] pub enum LoadError { # [doc = " An I/O error."] IO (std :: io :: Error) , # [doc = " An error within the scanner. This indicates a malformed YAML input."] Scan (ScanError) , # [doc = " A decoding error (e.g.: Invalid UTF-8)."] Decode (Cow < 'static , str >) , }
};
}
