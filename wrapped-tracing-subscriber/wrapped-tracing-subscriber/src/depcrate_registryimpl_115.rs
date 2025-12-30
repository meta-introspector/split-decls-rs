// Generated macro for impl_115 (impl)
macro_rules! Depcrate_registryimpl_115 {
() => {
// Module: crate::registry
// Provides: {"impl_115"}
// Dependencies: {}
impl < 'a , R > Iterator for Scope < 'a , R > where R : LookupSpan < 'a > , { type Item = SpanRef < 'a , R > ; fn next (& mut self) -> Option < Self :: Item > { loop { let curr = self . registry . span (self . next . as_ref () ?) ? ; # [cfg (all (feature = "registry" , feature = "std"))] let curr = curr . with_filter (self . filter) ; self . next = curr . data . parent () . cloned () ; # [cfg (all (feature = "registry" , feature = "std"))] { if ! curr . is_enabled_for (self . filter) { continue ; } } return Some (curr) ; } } }
};
}
