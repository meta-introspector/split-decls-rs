// Generated macro for convert (function)
macro_rules! Depcrateconvert {
() => {
// Module: crate
// Provides: {"convert"}
// Dependencies: {}
fn convert (time : i64) -> SYSTEMTIME { unsafe { let mut local = FILETIME :: default () ; FileTimeToLocalFileTime (& time as * const i64 as _ , & mut local) ; let mut time = SYSTEMTIME :: default () ; FileTimeToSystemTime (& local , & mut time) ; time } }
};
}
