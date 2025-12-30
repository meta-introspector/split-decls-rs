// Generated macro for extract_range_or_offset (function)
macro_rules! Depcrateextract_range_or_offset {
() => {
// Module: crate
// Provides: {"extract_range_or_offset"}
// Dependencies: {}
# [doc = " Extracts `TextRange` or `TextSize` depending on the amount of `$0` markers"] # [doc = " found in `text`."] # [doc = ""] # [doc = " # Panics"] # [doc = " Panics if no `$0` marker is present in the `text`."] pub fn extract_range_or_offset (text : & str) -> (RangeOrOffset , String) { if let Some ((range , text)) = try_extract_range (text) { return (RangeOrOffset :: Range (range) , text) ; } let (offset , text) = extract_offset (text) ; (RangeOrOffset :: Offset (offset) , text) }
};
}
