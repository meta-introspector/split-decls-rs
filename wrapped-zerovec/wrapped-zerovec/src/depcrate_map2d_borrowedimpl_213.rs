// Generated macro for impl_213 (impl)
macro_rules! Depcrate_map2d_borrowedimpl_213 {
() => {
// Module: crate::map2d::borrowed
// Provides: {"impl_213"}
// Dependencies: {}
impl < 'a , K0 , K1 , V > fmt :: Debug for ZeroMap2dBorrowed < 'a , K0 , K1 , V > where K0 : ZeroMapKV < 'a > + ? Sized , K1 : ZeroMapKV < 'a > + ? Sized , V : ZeroMapKV < 'a > + ? Sized , K0 :: Slice : fmt :: Debug , K1 :: Slice : fmt :: Debug , V :: Slice : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> Result < () , fmt :: Error > { f . debug_struct ("ZeroMap2dBorrowed") . field ("keys0" , & self . keys0) . field ("joiner" , & self . joiner) . field ("keys1" , & self . keys1) . field ("values" , & self . values) . finish () } }
};
}
