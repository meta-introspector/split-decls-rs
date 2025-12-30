// Generated macro for impl_344 (impl)
macro_rules! Depcrate_exprimpl_344 {
() => {
// Module: crate::expr
// Provides: {"impl_344"}
// Dependencies: {}
impl PartialEq for Member { fn eq (& self , other : & Self) -> bool { match (self , other) { (Member :: Named (this) , Member :: Named (other)) => this == other , (Member :: Unnamed (this) , Member :: Unnamed (other)) => this == other , _ => false , } } }
};
}
