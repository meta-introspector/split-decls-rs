// Generated macro for try_extract_range (function)
macro_rules! Depcratetry_extract_range {
() => {
// Module: crate
// Provides: {"try_extract_range"}
// Dependencies: {}
# [doc = " Returns `TextRange` between the first two markers `$0...$0` and the copy"] # [doc = " of `text` without both of these markers."] fn try_extract_range (text : & str) -> Option < (TextRange , String) > { let (start , text) = try_extract_offset (text) ? ; let (end , text) = try_extract_offset (& text) ? ; Some ((TextRange :: new (start , end) , text)) }
};
}
