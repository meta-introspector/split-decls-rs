// Generated macro for impl_249 (impl)
macro_rules! Depcrate_map2d_mapimpl_249 {
() => {
// Module: crate::map2d::map
// Provides: {"impl_249"}
// Dependencies: {}
impl < 'a , 'b , K0 , K1 , V > PartialEq < ZeroMap2d < 'b , K0 , K1 , V > > for ZeroMap2d < 'a , K0 , K1 , V > where K0 : for < 'c > ZeroMapKV < 'c > + ? Sized , K1 : for < 'c > ZeroMapKV < 'c > + ? Sized , V : for < 'c > ZeroMapKV < 'c > + ? Sized , < K0 as ZeroMapKV < 'a > > :: Container : PartialEq < < K0 as ZeroMapKV < 'b > > :: Container > , < K1 as ZeroMapKV < 'a > > :: Container : PartialEq < < K1 as ZeroMapKV < 'b > > :: Container > , < V as ZeroMapKV < 'a > > :: Container : PartialEq < < V as ZeroMapKV < 'b > > :: Container > , { fn eq (& self , other : & ZeroMap2d < 'b , K0 , K1 , V >) -> bool { self . keys0 . eq (& other . keys0) && self . joiner . eq (& other . joiner) && self . keys1 . eq (& other . keys1) && self . values . eq (& other . values) } }
};
}
