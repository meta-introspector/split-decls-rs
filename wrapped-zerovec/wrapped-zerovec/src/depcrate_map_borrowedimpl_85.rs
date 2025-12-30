// Generated macro for impl_85 (impl)
macro_rules! Depcrate_map_borrowedimpl_85 {
() => {
// Module: crate::map::borrowed
// Provides: {"impl_85"}
// Dependencies: {}
impl < 'a , 'b , K , V > PartialEq < ZeroMapBorrowed < 'b , K , V > > for ZeroMapBorrowed < 'a , K , V > where K : for < 'c > ZeroMapKV < 'c > + ? Sized , V : for < 'c > ZeroMapKV < 'c > + ? Sized , < K as ZeroMapKV < 'a > > :: Slice : PartialEq < < K as ZeroMapKV < 'b > > :: Slice > , < V as ZeroMapKV < 'a > > :: Slice : PartialEq < < V as ZeroMapKV < 'b > > :: Slice > , { fn eq (& self , other : & ZeroMapBorrowed < 'b , K , V >) -> bool { self . keys . eq (other . keys) && self . values . eq (other . values) } }
};
}
