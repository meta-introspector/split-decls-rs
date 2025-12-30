// Generated macro for impl_229 (impl)
macro_rules! Depcrate_map2d_cursorimpl_229 {
() => {
// Module: crate::map2d::cursor
// Provides: {"impl_229"}
// Dependencies: {}
impl < 'l , 'a , K0 , K1 , V > fmt :: Debug for ZeroMap2dCursor < 'l , 'a , K0 , K1 , V > where K0 : ZeroMapKV < 'a > + ? Sized , K1 : ZeroMapKV < 'a > + ? Sized , V : ZeroMapKV < 'a > + ? Sized , K0 :: Slice : fmt :: Debug , K1 :: Slice : fmt :: Debug , V :: Slice : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> Result < () , fmt :: Error > { f . debug_struct ("ZeroMap2d") . field ("keys0" , & self . keys0) . field ("joiner" , & self . joiner) . field ("keys1" , & self . keys1) . field ("values" , & self . values) . field ("key0_index" , & self . key0_index) . finish () } }
};
}
