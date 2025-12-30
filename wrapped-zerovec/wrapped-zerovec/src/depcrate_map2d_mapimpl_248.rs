// Generated macro for impl_248 (impl)
macro_rules! Depcrate_map2d_mapimpl_248 {
() => {
// Module: crate::map2d::map
// Provides: {"impl_248"}
// Dependencies: {}
impl < 'a , K0 , K1 , V > From < ZeroMap2dBorrowed < 'a , K0 , K1 , V > > for ZeroMap2d < 'a , K0 , K1 , V > where K0 : ZeroMapKV < 'a > , K1 : ZeroMapKV < 'a > , V : ZeroMapKV < 'a > , K0 : ? Sized , K1 : ? Sized , V : ? Sized , { fn from (other : ZeroMap2dBorrowed < 'a , K0 , K1 , V >) -> Self { Self { keys0 : K0 :: Container :: zvl_from_borrowed (other . keys0) , joiner : other . joiner . as_zerovec () , keys1 : K1 :: Container :: zvl_from_borrowed (other . keys1) , values : V :: Container :: zvl_from_borrowed (other . values) , } } }
};
}
