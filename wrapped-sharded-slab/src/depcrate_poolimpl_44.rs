// Generated macro for impl_44 (impl)
macro_rules! Depcrate_poolimpl_44 {
() => {
// Module: crate::pool
// Provides: {"impl_44"}
// Dependencies: {}
impl < T , C > PartialEq < T > for OwnedRefMut < T , C > where T : PartialEq < T > + Clear + Default , C : cfg :: Config , { fn eq (& self , other : & T) -> bool { * self . value () == * other } }
};
}
