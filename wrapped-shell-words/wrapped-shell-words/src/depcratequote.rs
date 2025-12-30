// Generated macro for quote (function)
macro_rules! Depcratequote {
() => {
// Module: crate
// Provides: {"quote"}
// Dependencies: {}
# [doc = " Escapes special characters in a string, so that it will retain its literal"] # [doc = " meaning when used as a part of command in Unix shell."] # [doc = ""] # [doc = " It tries to avoid introducing any unnecessary quotes or escape characters,"] # [doc = " but specifics regarding quoting style are left unspecified."] pub fn quote (s : & str) -> Cow < str > { match escape_style (s) { EscapeStyle :: None => s . into () , EscapeStyle :: SingleQuoted => format ! ("'{}'" , s) . into () , EscapeStyle :: Mixed => { let mut quoted = String :: new () ; quoted . push ('\'') ; for c in s . chars () { if c == '\'' { quoted . push_str ("'\\''") ; } else { quoted . push (c) ; } } quoted . push ('\'') ; quoted . into () } } }
};
}
