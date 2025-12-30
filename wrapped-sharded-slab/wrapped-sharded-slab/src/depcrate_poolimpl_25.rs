// Generated macro for impl_25 (impl)
macro_rules! Depcrate_poolimpl_25 {
() => {
// Module: crate::pool
// Provides: {"impl_25"}
// Dependencies: {}
impl < T , C > PartialEq < T > for Ref < '_ , T , C > where T : PartialEq < T > + Clear + Default , C : cfg :: Config , { fn eq (& self , other : & T) -> bool { * self . value () == * other } }
};
}
