// Generated macro for impl_923 (impl)
macro_rules! Depcrate_punctuatedimpl_923 {
() => {
// Module: crate::punctuated
// Provides: {"impl_923"}
// Dependencies: {}
impl < T , P > Index < usize > for Punctuated < T , P > { type Output = T ; fn index (& self , index : usize) -> & Self :: Output { if index == self . len () - 1 { match & self . last { Some (t) => t , None => & self . inner [index] . 0 , } } else { & self . inner [index] . 0 } } }
};
}
