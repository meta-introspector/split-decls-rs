// Generated macro for utf16_str (function)
macro_rules! Depcrate_windows_systemutf16_str {
() => {
// Module: crate::windows::system
// Provides: {"utf16_str"}
// Dependencies: {}
fn utf16_str < S : AsRef < OsStr > + ? Sized > (text : & S) -> Vec < u16 > { OsStr :: new (text) . encode_wide () . chain (Some (0)) . collect :: < Vec < _ > > () }
};
}
