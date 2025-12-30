// Generated macro for impl_227 (impl)
macro_rules! Depcrate_map2d_cursorimpl_227 {
() => {
// Module: crate::map2d::cursor
// Provides: {"impl_227"}
// Dependencies: {}
impl < 'l , 'a , K0 , K1 , V > ZeroMap2dCursor < 'l , 'a , K0 , K1 , V > where K0 : ZeroMapKV < 'a > , K1 : ZeroMapKV < 'a > + Ord , V : ZeroMapKV < 'a > , V : Copy , K0 : ? Sized , K1 : ? Sized , { # [doc = " For cases when `V` is fixed-size, obtain a direct copy of `V` instead of `V::ULE`"] # [doc = ""] # [doc = " ```rust"] # [doc = " use zerovec::ZeroMap2d;"] # [doc = ""] # [doc = " let mut map: ZeroMap2d<u16, u16, u16> = ZeroMap2d::new();"] # [doc = " map.insert(&1, &2, &3);"] # [doc = " map.insert(&1, &4, &5);"] # [doc = " map.insert(&6, &7, &8);"] # [doc = ""] # [doc = " assert_eq!(map.get0(&6).unwrap().get1_copied(&7), Some(8));"] # [doc = " ```"] # [inline] pub fn get1_copied (& self , key1 : & K1) -> Option < V > { let key1_index = self . get_key1_index (key1) ? ; self . get1_copied_at (key1_index) } # [doc = " For cases when `V` is fixed-size, obtain a direct copy of `V` instead of `V::ULE`"] # [inline] pub fn get1_copied_by (& self , predicate : impl FnMut (& K1) -> Ordering) -> Option < V > { let key1_index = self . get_key1_index_by (predicate) ? ; self . get1_copied_at (key1_index) } }
};
}
