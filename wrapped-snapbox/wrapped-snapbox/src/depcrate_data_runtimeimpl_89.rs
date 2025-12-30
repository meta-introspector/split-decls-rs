// Generated macro for impl_89 (impl)
macro_rules! Depcrate_data_runtimeimpl_89 {
() => {
// Module: crate::data::runtime
// Provides: {"impl_89"}
// Dependencies: {}
impl SourceFileRuntime { fn new (inline : & Inline) -> std :: io :: Result < SourceFileRuntime > { let path = inline . position . file . clone () ; let original_text = std :: fs :: read_to_string (& path) ? ; let patchwork = Patchwork :: new (original_text . clone ()) ; Ok (SourceFileRuntime { path , original_text , patchwork , }) } fn update (& mut self , actual : & str , inline : & Inline) -> std :: io :: Result < () > { let span = Span :: from_pos (& inline . position , & self . original_text) ; let patch = format_patch (actual) ; self . patchwork . patch (span . literal_range , & patch) ? ; std :: fs :: write (& inline . position . file , & self . patchwork . text) } }
};
}
