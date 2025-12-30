// Generated macro for slice_has_bom (function)
macro_rules! Depcrate_searcherslice_has_bom {
() => {
// Module: crate::searcher
// Provides: {"slice_has_bom"}
// Dependencies: {}
# [doc = " Returns true if and only if the given slice begins with a UTF-8 or UTF-16"] # [doc = " BOM."] # [doc = ""] # [doc = " This is used by the searcher to determine if a transcoder is necessary."] # [doc = " Otherwise, it is advantageous to search the slice directly."] fn slice_has_bom (slice : & [u8]) -> bool { let enc = match encoding_rs :: Encoding :: for_bom (slice) { None => return false , Some ((enc , _)) => enc , } ; log :: trace ! ("found byte-order mark (BOM) for encoding {enc:?}") ; [encoding_rs :: UTF_16LE , encoding_rs :: UTF_16BE , encoding_rs :: UTF_8] . contains (& enc) }
};
}
