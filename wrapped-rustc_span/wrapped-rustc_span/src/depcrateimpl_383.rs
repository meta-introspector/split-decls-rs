// Generated macro for impl_383 (impl)
macro_rules! Depcrateimpl_383 {
() => {
// Module: crate
// Provides: {"impl_383"}
// Dependencies: {}
impl ExternalSource { pub fn get_source (& self) -> Option < & str > { match self { ExternalSource :: Foreign { kind : ExternalSourceKind :: Present (src) , .. } => Some (src) , _ => None , } } }
};
}
