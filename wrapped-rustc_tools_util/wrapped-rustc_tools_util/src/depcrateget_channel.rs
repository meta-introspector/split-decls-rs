// Generated macro for get_channel (function)
macro_rules! Depcrateget_channel {
() => {
// Module: crate
// Provides: {"get_channel"}
// Dependencies: {}
# [must_use] pub fn get_channel (compiler_version : Option < String >) -> String { if let Ok (channel) = std :: env :: var ("CFG_RELEASE_CHANNEL") { return channel ; } if let Some (rustc_output) = compiler_version { if rustc_output . contains ("beta") { return String :: from ("beta") ; } else if rustc_output . contains ("nightly") { return String :: from ("nightly") ; } else if rustc_output . contains ("dev") { return String :: from ("dev") ; } } String :: from ("stable") }
};
}
