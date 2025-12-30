// Generated macro for impl_399 (impl)
macro_rules! Depcrateimpl_399 {
() => {
// Module: crate
// Provides: {"impl_399"}
// Dependencies: {}
impl ExternalSource { pub fn get_source (& self) -> Option < & str > { match self { ExternalSource :: Foreign { kind : ExternalSourceKind :: Present (src) , .. } => Some (src) , _ => None , } } }
};
}
