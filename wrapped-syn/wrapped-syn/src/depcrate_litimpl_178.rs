// Generated macro for impl_178 (impl)
macro_rules! Depcrate_litimpl_178 {
() => {
// Module: crate::lit
// Provides: {"impl_178"}
// Dependencies: {}
impl PartialEq for LitKind { fn eq (& self , other : & LitKind) -> bool { match (self , other) { (& LitKind :: Bool (b1) , & LitKind :: Bool (b2)) => b1 == b2 , (& LitKind :: Other (ref l1) , & LitKind :: Other (ref l2)) => { l1 . to_string () == l2 . to_string () } _ => false , } } }
};
}
