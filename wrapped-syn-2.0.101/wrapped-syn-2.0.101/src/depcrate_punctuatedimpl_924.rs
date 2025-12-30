// Generated macro for impl_924 (impl)
macro_rules! Depcrate_punctuatedimpl_924 {
() => {
// Module: crate::punctuated
// Provides: {"impl_924"}
// Dependencies: {}
impl < T , P > IndexMut < usize > for Punctuated < T , P > { fn index_mut (& mut self , index : usize) -> & mut Self :: Output { if index == self . len () - 1 { match & mut self . last { Some (t) => t , None => & mut self . inner [index] . 0 , } } else { & mut self . inner [index] . 0 } } }
};
}
