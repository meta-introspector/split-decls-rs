// Generated macro for impl_940 (impl)
macro_rules! Depcrate_punctuatedimpl_940 {
() => {
// Module: crate::punctuated
// Provides: {"impl_940"}
// Dependencies: {}
impl < T , P > Index < usize > for Punctuated < T , P > { type Output = T ; fn index (& self , index : usize) -> & Self :: Output { if index . checked_add (1) == Some (self . len ()) { match & self . last { Some (t) => t , None => & self . inner [index] . 0 , } } else { & self . inner [index] . 0 } } }
};
}
