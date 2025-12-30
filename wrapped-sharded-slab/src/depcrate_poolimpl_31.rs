// Generated macro for impl_31 (impl)
macro_rules! Depcrate_poolimpl_31 {
() => {
// Module: crate::pool
// Provides: {"impl_31"}
// Dependencies: {}
impl < T , C > PartialEq < T > for RefMut < '_ , T , C > where T : PartialEq < T > + Clear + Default , C : cfg :: Config , { fn eq (& self , other : & T) -> bool { self . value () . eq (other) } }
};
}
