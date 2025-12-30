// Generated macro for impl_250 (impl)
macro_rules! Depcrate_map2d_mapimpl_250 {
() => {
// Module: crate::map2d::map
// Provides: {"impl_250"}
// Dependencies: {}
impl < 'a , K0 , K1 , V > fmt :: Debug for ZeroMap2d < 'a , K0 , K1 , V > where K0 : ZeroMapKV < 'a > + ? Sized , K1 : ZeroMapKV < 'a > + ? Sized , V : ZeroMapKV < 'a > + ? Sized , < K0 as ZeroMapKV < 'a > > :: Container : fmt :: Debug , < K1 as ZeroMapKV < 'a > > :: Container : fmt :: Debug , < V as ZeroMapKV < 'a > > :: Container : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> Result < () , fmt :: Error > { f . debug_struct ("ZeroMap2d") . field ("keys0" , & self . keys0) . field ("joiner" , & self . joiner) . field ("keys1" , & self . keys1) . field ("values" , & self . values) . finish () } }
};
}
