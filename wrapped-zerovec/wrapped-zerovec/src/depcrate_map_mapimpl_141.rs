// Generated macro for impl_141 (impl)
macro_rules! Depcrate_map_mapimpl_141 {
() => {
// Module: crate::map::map
// Provides: {"impl_141"}
// Dependencies: {}
impl < 'a , K , V > Clone for ZeroMap < 'a , K , V > where K : ZeroMapKV < 'a > + ? Sized , V : ZeroMapKV < 'a > + ? Sized , < K as ZeroMapKV < 'a > > :: Container : Clone , < V as ZeroMapKV < 'a > > :: Container : Clone , { fn clone (& self) -> Self { Self { keys : self . keys . clone () , values : self . values . clone () , } } }
};
}
