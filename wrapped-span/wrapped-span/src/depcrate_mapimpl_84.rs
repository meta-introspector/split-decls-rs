// Generated macro for impl_84 (impl)
macro_rules! Depcrate_mapimpl_84 {
() => {
// Module: crate::map
// Provides: {"impl_84"}
// Dependencies: {}
impl RealSpanMap { # [doc = " Creates a real file span map that returns absolute ranges (relative ranges to the root ast id)."] pub fn absolute (file_id : EditionedFileId) -> Self { RealSpanMap { file_id , pairs : Box :: from ([(TextSize :: new (0) , ROOT_ERASED_FILE_AST_ID)]) , end : TextSize :: new (! 0) , } } pub fn from_file (file_id : EditionedFileId , pairs : Box < [(TextSize , ErasedFileAstId)] > , end : TextSize ,) -> Self { Self { file_id , pairs , end } } pub fn span_for_range (& self , range : TextRange) -> Span { assert ! (range . end () <= self . end , "range {range:?} goes beyond the end of the file {:?}" , self . end) ; let start = range . start () ; let idx = self . pairs . binary_search_by (| & (it , _) | it . cmp (& start) . then (std :: cmp :: Ordering :: Less)) . unwrap_err () ; let (offset , ast_id) = self . pairs [idx - 1] ; Span { range : range - offset , anchor : SpanAnchor { file_id : self . file_id , ast_id } , ctx : SyntaxContext :: root (self . file_id . edition ()) , } } }
};
}
