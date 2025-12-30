// Generated macro for impl_103 (impl)
macro_rules! Depcrate_linked_listimpl_103 {
() => {
// Module: crate::linked_list
// Provides: {"impl_103"}
// Dependencies: {}
impl < T : Display > Display for LinkedEntry < T > { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if let Some (instance) = self . instance . as_ref () { write ! (f , "Some({instance})") } else { write ! (f , "None") } } }
};
}
