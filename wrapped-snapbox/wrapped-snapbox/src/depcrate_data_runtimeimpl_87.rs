// Generated macro for impl_87 (impl)
macro_rules! Depcrate_data_runtimeimpl_87 {
() => {
// Module: crate::data::runtime
// Provides: {"impl_87"}
// Dependencies: {}
impl Runtime { const fn new () -> Self { Self { per_file : Vec :: new () , path_count : Vec :: new () , } } pub (crate) fn count (& mut self , path_prefix : & str) -> usize { if let Some (entry) = self . path_count . iter_mut () . find (| entry | entry . is (path_prefix)) { entry . next () } else { let entry = PathRuntime :: new (path_prefix) ; let next = entry . count () ; self . path_count . push (entry) ; next } } pub (crate) fn write (& mut self , actual : & Data , inline : & Inline) -> std :: io :: Result < () > { let actual = actual . render () . expect ("`actual` must be UTF-8") ; if let Some (entry) = self . per_file . iter_mut () . find (| f | f . path == inline . position . file) { entry . update (& actual , inline) ? ; } else { let mut entry = SourceFileRuntime :: new (inline) ? ; entry . update (& actual , inline) ? ; self . per_file . push (entry) ; } Ok (()) } }
};
}
