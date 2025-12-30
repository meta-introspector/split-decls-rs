// Generated macro for version (function)
macro_rules! Depcrateversion {
() => {
// Module: crate
// Provides: {"version"}
// Dependencies: {}
fn version () -> String { let mut out = String :: new () ; out . push_str ("pub const UNICODE_VERSION: (u8, u8, u8) = ") ; let readme = std :: fs :: read_to_string (std :: path :: Path :: new (UNICODE_DIRECTORY) . join ("ReadMe.txt")) . unwrap () ; let prefix = "for Version " ; let start = readme . find (prefix) . unwrap () + prefix . len () ; let end = readme . find (" of the Unicode Standard.") . unwrap () ; let version = readme [start .. end] . split ('.') . map (| v | v . parse :: < u32 > () . expect (v)) . collect :: < Vec < _ > > () ; let [major , minor , micro] = [version [0] , version [1] , version [2]] ; out . push_str (& format ! ("({major}, {minor}, {micro});\n")) ; out }
};
}
