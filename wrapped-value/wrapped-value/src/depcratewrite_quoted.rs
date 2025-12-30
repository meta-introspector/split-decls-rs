// Generated macro for write_quoted (function)
macro_rules! Depcratewrite_quoted {
() => {
// Module: crate
// Provides: {"write_quoted"}
// Dependencies: {}
fn write_quoted (s : & str , f : & mut Formatter < '_ >) -> fmt :: Result { f . write_char ('"') ? ; for c in s . chars () { match c { '\r' => f . write_str ("\\r") , '\n' => f . write_str ("\\n") , '\t' => f . write_str ("\\t") , '"' => f . write_str ("\\\"") , '\\' => f . write_str ("\\\\") , c if c . is_control () => write ! (f , "\\u{:04}" , c as u32) , c => f . write_char (c) , } ? } f . write_char ('"') }
};
}
