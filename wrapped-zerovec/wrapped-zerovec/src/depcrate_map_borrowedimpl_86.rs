// Generated macro for impl_86 (impl)
macro_rules! Depcrate_map_borrowedimpl_86 {
() => {
// Module: crate::map::borrowed
// Provides: {"impl_86"}
// Dependencies: {}
impl < 'a , K , V > fmt :: Debug for ZeroMapBorrowed < 'a , K , V > where K : ZeroMapKV < 'a > + ? Sized , V : ZeroMapKV < 'a > + ? Sized , K :: Slice : fmt :: Debug , V :: Slice : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> Result < () , fmt :: Error > { f . debug_struct ("ZeroMapBorrowed") . field ("keys" , & self . keys) . field ("values" , & self . values) . finish () } }
};
}
