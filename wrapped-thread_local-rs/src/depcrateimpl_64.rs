// Generated macro for impl_64 (impl)
macro_rules! Depcrateimpl_64 {
() => {
// Module: crate
// Provides: {"impl_64"}
// Dependencies: {}
impl < 'a , T : Send + fmt :: Debug > fmt :: Debug for IterMut < 'a , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("IterMut") . field ("raw" , & self . raw) . finish () } }
};
}
