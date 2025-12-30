// Generated macro for impl_139 (impl)
macro_rules! Depcrate_map_mapimpl_139 {
() => {
// Module: crate::map::map
// Provides: {"impl_139"}
// Dependencies: {}
impl < 'a , 'b , K , V > PartialEq < ZeroMap < 'b , K , V > > for ZeroMap < 'a , K , V > where K : for < 'c > ZeroMapKV < 'c > + ? Sized , V : for < 'c > ZeroMapKV < 'c > + ? Sized , < K as ZeroMapKV < 'a > > :: Container : PartialEq < < K as ZeroMapKV < 'b > > :: Container > , < V as ZeroMapKV < 'a > > :: Container : PartialEq < < V as ZeroMapKV < 'b > > :: Container > , { fn eq (& self , other : & ZeroMap < 'b , K , V >) -> bool { self . keys . eq (& other . keys) && self . values . eq (& other . values) } }
};
}
