// Generated macro for impl_102 (impl)
macro_rules! Depcrate_valueimpl_102 {
() => {
// Module: crate::value
// Provides: {"impl_102"}
// Dependencies: {}
impl Index for str { fn index < 'a > (& self , val : & 'a Value) -> Option < & 'a Value > { match * val { Value :: Table (ref a) => a . get (self) , _ => None , } } fn index_mut < 'a > (& self , val : & 'a mut Value) -> Option < & 'a mut Value > { match * val { Value :: Table (ref mut a) => a . get_mut (self) , _ => None , } } }
};
}
