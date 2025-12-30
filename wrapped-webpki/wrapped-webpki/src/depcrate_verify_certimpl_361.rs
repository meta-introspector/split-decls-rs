// Generated macro for impl_361 (impl)
macro_rules! Depcrate_verify_certimpl_361 {
() => {
// Module: crate::verify_cert
// Provides: {"impl_361"}
// Dependencies: {}
impl < 'a > Iterator for PathIter < 'a > { type Item = PathNode < 'a > ; fn next (& mut self) -> Option < Self :: Item > { let next = self . next ? ; self . next = match next { 0 => None , _ => Some (next - 1) , } ; Some (PathNode { path : self . path , index : next , cert : self . path . get (next) , }) } }
};
}
