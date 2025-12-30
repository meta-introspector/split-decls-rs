// Generated macro for file_contains (function)
macro_rules! Depcrate_caching_source_map_viewfile_contains {
() => {
// Module: crate::caching_source_map_view
// Provides: {"file_contains"}
// Dependencies: {}
# [inline] fn file_contains (file : & SourceFile , pos : BytePos) -> bool { file . contains (pos) && ! file . is_empty () }
};
}
