// Generated macro for impl_346 (impl)
macro_rules! Depcrate_exprimpl_346 {
() => {
// Module: crate::expr
// Provides: {"impl_346"}
// Dependencies: {}
impl PartialEq for Member { fn eq (& self , other : & Self) -> bool { match (self , other) { (Member :: Named (this) , Member :: Named (other)) => this == other , (Member :: Unnamed (this) , Member :: Unnamed (other)) => this == other , _ => false , } } }
};
}
