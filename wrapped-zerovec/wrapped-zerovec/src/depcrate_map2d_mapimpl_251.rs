// Generated macro for impl_251 (impl)
macro_rules! Depcrate_map2d_mapimpl_251 {
() => {
// Module: crate::map2d::map
// Provides: {"impl_251"}
// Dependencies: {}
impl < 'a , K0 , K1 , V > Clone for ZeroMap2d < 'a , K0 , K1 , V > where K0 : ZeroMapKV < 'a > + ? Sized , K1 : ZeroMapKV < 'a > + ? Sized , V : ZeroMapKV < 'a > + ? Sized , < K0 as ZeroMapKV < 'a > > :: Container : Clone , < K1 as ZeroMapKV < 'a > > :: Container : Clone , < V as ZeroMapKV < 'a > > :: Container : Clone , { fn clone (& self) -> Self { Self { keys0 : self . keys0 . clone () , joiner : self . joiner . clone () , keys1 : self . keys1 . clone () , values : self . values . clone () , } } }
};
}
