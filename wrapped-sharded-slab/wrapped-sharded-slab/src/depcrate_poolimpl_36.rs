// Generated macro for impl_36 (impl)
macro_rules! Depcrate_poolimpl_36 {
() => {
// Module: crate::pool
// Provides: {"impl_36"}
// Dependencies: {}
impl < T , C > PartialEq < T > for OwnedRef < T , C > where T : PartialEq < T > + Clear + Default , C : cfg :: Config , { fn eq (& self , other : & T) -> bool { * self . value () == * other } }
};
}
