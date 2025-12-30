// Generated macro for impl_212 (impl)
macro_rules! Depcrate_map2d_borrowedimpl_212 {
() => {
// Module: crate::map2d::borrowed
// Provides: {"impl_212"}
// Dependencies: {}
impl < 'a , 'b , K0 , K1 , V > PartialEq < ZeroMap2dBorrowed < 'b , K0 , K1 , V > > for ZeroMap2dBorrowed < 'a , K0 , K1 , V > where K0 : for < 'c > ZeroMapKV < 'c > + ? Sized , K1 : for < 'c > ZeroMapKV < 'c > + ? Sized , V : for < 'c > ZeroMapKV < 'c > + ? Sized , < K0 as ZeroMapKV < 'a > > :: Slice : PartialEq < < K0 as ZeroMapKV < 'b > > :: Slice > , < K1 as ZeroMapKV < 'a > > :: Slice : PartialEq < < K1 as ZeroMapKV < 'b > > :: Slice > , < V as ZeroMapKV < 'a > > :: Slice : PartialEq < < V as ZeroMapKV < 'b > > :: Slice > , { fn eq (& self , other : & ZeroMap2dBorrowed < 'b , K0 , K1 , V >) -> bool { self . keys0 . eq (other . keys0) && self . joiner . eq (other . joiner) && self . keys1 . eq (other . keys1) && self . values . eq (other . values) } }
};
}
