// Generated macro for impl_50 (impl)
macro_rules! Depcrate_stringimpl_50 {
() => {
// Module: crate::string
// Provides: {"impl_50"}
// Dependencies: {}
impl KeyMetrics { fn new () -> Self { Self { unquoted : true , single_quotes : false , double_quotes : false , escape_codes : false , escape : false , } } fn calculate (s : & str) -> Self { let mut metrics = Self :: new () ; metrics . unquoted = ! s . is_empty () ; for byte in s . as_bytes () { if ! matches ! (* byte , b'a' ..= b'z' | b'A' ..= b'Z' | b'0' ..= b'9' | b'-' | b'_') { metrics . unquoted = false ; } match * byte { b'\'' => metrics . single_quotes = true , b'"' => metrics . double_quotes = true , b'\\' => metrics . escape = true , b'\t' => { } c if c <= 0x1f || c == 0x7f => metrics . escape_codes = true , _ => { } } } metrics } }
};
}
