// Generated macro for impl_223 (impl)
macro_rules! Depcrate_map2d_cursorimpl_223 {
() => {
// Module: crate::map2d::cursor
// Provides: {"impl_223"}
// Dependencies: {}
impl < 'a , K0 , K1 , V > ZeroMap2dCursor < 'a , 'a , K0 , K1 , V > where K0 : ZeroMapKV < 'a > , K1 : ZeroMapKV < 'a > , V : ZeroMapKV < 'a > , K0 : ? Sized , K1 : ? Sized , V : ? Sized , { # [doc = " `key0_index` must be in range"] pub (crate) fn from_borrowed (borrowed : & ZeroMap2dBorrowed < 'a , K0 , K1 , V > , key0_index : usize ,) -> Self { debug_assert ! (key0_index < borrowed . joiner . len ()) ; ZeroMap2dCursor { keys0 : borrowed . keys0 , joiner : borrowed . joiner , keys1 : borrowed . keys1 , values : borrowed . values , key0_index , } } }
};
}
