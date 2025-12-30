// Generated macro for impl_303 (impl)
macro_rules! Depcrate_source_trackerimpl_303 {
() => {
// Module: crate::source_tracker
// Provides: {"impl_303"}
// Dependencies: {}
impl SourceMap { pub fn new () -> Self { Self { mappings : HashMap :: new () , } } pub fn add_mapping (& mut self , offset : usize , location : SourceLocation) { self . mappings . insert (offset , location) ; } pub fn get_location (& self , offset : usize) -> Option < & SourceLocation > { self . mappings . get (& offset) } }
};
}
