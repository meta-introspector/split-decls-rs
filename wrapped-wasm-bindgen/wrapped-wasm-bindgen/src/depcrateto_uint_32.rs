// Generated macro for to_uint_32 (function)
macro_rules! Depcrateto_uint_32 {
() => {
// Module: crate
// Provides: {"to_uint_32"}
// Dependencies: {}
fn to_uint_32 (v : & JsValue) -> Option < u32 > { v . as_f64 () . map (| n | { if n . is_infinite () { 0 } else { (n as i64) as u32 } }) }
};
}
