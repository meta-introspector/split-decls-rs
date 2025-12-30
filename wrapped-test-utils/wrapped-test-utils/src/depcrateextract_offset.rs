// Generated macro for extract_offset (function)
macro_rules! Depcrateextract_offset {
() => {
// Module: crate
// Provides: {"extract_offset"}
// Dependencies: {}
# [doc = " Infallible version of `try_extract_offset()`."] pub fn extract_offset (text : & str) -> (TextSize , String) { match try_extract_offset (text) { None => panic ! ("text should contain cursor marker") , Some (result) => result , } }
};
}
