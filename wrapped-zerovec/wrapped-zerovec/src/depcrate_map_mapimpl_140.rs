// Generated macro for impl_140 (impl)
macro_rules! Depcrate_map_mapimpl_140 {
() => {
// Module: crate::map::map
// Provides: {"impl_140"}
// Dependencies: {}
impl < 'a , K , V > fmt :: Debug for ZeroMap < 'a , K , V > where K : ZeroMapKV < 'a > + ? Sized , V : ZeroMapKV < 'a > + ? Sized , < K as ZeroMapKV < 'a > > :: Container : fmt :: Debug , < V as ZeroMapKV < 'a > > :: Container : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> Result < () , fmt :: Error > { f . debug_struct ("ZeroMap") . field ("keys" , & self . keys) . field ("values" , & self . values) . finish () } }
};
}
