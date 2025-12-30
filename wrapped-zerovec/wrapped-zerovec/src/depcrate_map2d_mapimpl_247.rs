// Generated macro for impl_247 (impl)
macro_rules! Depcrate_map2d_mapimpl_247 {
() => {
// Module: crate::map2d::map
// Provides: {"impl_247"}
// Dependencies: {}
impl < 'a , K0 , K1 , V > ZeroMap2d < 'a , K0 , K1 , V > where K0 : ZeroMapKV < 'a > + Ord , K1 : ZeroMapKV < 'a > + Ord , V : ZeroMapKV < 'a > , V : Copy , K0 : ? Sized , K1 : ? Sized , { # [doc = " For cases when `V` is fixed-size, obtain a direct copy of `V` instead of `V::ULE`"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # use zerovec::ZeroMap2d;"] # [doc = " let mut map: ZeroMap2d<u16, u16, u16> = ZeroMap2d::new();"] # [doc = " map.insert(&1, &2, &3);"] # [doc = " map.insert(&1, &4, &5);"] # [doc = " map.insert(&6, &7, &8);"] # [doc = ""] # [doc = " assert_eq!(map.get_copied_2d(&6, &7), Some(8));"] # [doc = " ```"] # [inline] pub fn get_copied_2d (& self , key0 : & K0 , key1 : & K1) -> Option < V > { self . get0 (key0) ? . get1_copied (key1) } }
};
}
