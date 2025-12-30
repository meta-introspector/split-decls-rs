// Generated macro for impl_941 (impl)
macro_rules! Depcrate_punctuatedimpl_941 {
() => {
// Module: crate::punctuated
// Provides: {"impl_941"}
// Dependencies: {}
impl < T , P > IndexMut < usize > for Punctuated < T , P > { fn index_mut (& mut self , index : usize) -> & mut Self :: Output { if index . checked_add (1) == Some (self . len ()) { match & mut self . last { Some (t) => t , None => & mut self . inner [index] . 0 , } } else { & mut self . inner [index] . 0 } } }
};
}
