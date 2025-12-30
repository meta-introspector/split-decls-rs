// Generated macro for impl_228 (impl)
macro_rules! Depcrate_map2d_cursorimpl_228 {
() => {
// Module: crate::map2d::cursor
// Provides: {"impl_228"}
// Dependencies: {}
impl < 'm , 'n , 'a , 'b , K0 , K1 , V > PartialEq < ZeroMap2dCursor < 'n , 'b , K0 , K1 , V > > for ZeroMap2dCursor < 'm , 'a , K0 , K1 , V > where K0 : for < 'c > ZeroMapKV < 'c > + ? Sized , K1 : for < 'c > ZeroMapKV < 'c > + ? Sized , V : for < 'c > ZeroMapKV < 'c > + ? Sized , < K0 as ZeroMapKV < 'a > > :: Slice : PartialEq < < K0 as ZeroMapKV < 'b > > :: Slice > , < K1 as ZeroMapKV < 'a > > :: Slice : PartialEq < < K1 as ZeroMapKV < 'b > > :: Slice > , < V as ZeroMapKV < 'a > > :: Slice : PartialEq < < V as ZeroMapKV < 'b > > :: Slice > , { fn eq (& self , other : & ZeroMap2dCursor < 'n , 'b , K0 , K1 , V >) -> bool { self . keys0 . eq (other . keys0) && self . joiner . eq (other . joiner) && self . keys1 . eq (other . keys1) && self . values . eq (other . values) && self . key0_index . eq (& other . key0_index) } }
};
}
