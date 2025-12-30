// Generated macro for impl_208 (impl)
macro_rules! Depcrate_map2d_borrowedimpl_208 {
() => {
// Module: crate::map2d::borrowed
// Provides: {"impl_208"}
// Dependencies: {}
impl < 'a , K0 , K1 , V > ZeroMap2dBorrowed < 'a , K0 , K1 , V > where K0 : ZeroMapKV < 'a > + Ord , K1 : ZeroMapKV < 'a > + Ord , V : ZeroMapKV < 'a > , K0 : ? Sized , K1 : ? Sized , V : ? Sized , { # [doc = " Get the value associated with `key0` and `key1`, if it exists."] # [doc = ""] # [doc = " This is able to return values that live longer than the map itself"] # [doc = " since they borrow directly from the backing buffer. This is the"] # [doc = " primary advantage of using [`ZeroMap2dBorrowed`](super::ZeroMap2dBorrowed) over [`ZeroMap2d`](super::ZeroMap2d)."] # [doc = ""] # [doc = " ```rust"] # [doc = " use zerovec::ZeroMap2d;"] # [doc = ""] # [doc = " let mut map = ZeroMap2d::new();"] # [doc = " map.insert(&1, \"one\", \"foo\");"] # [doc = " map.insert(&2, \"one\", \"bar\");"] # [doc = " map.insert(&2, \"two\", \"baz\");"] # [doc = ""] # [doc = " let borrowed = map.as_borrowed();"] # [doc = " assert_eq!(borrowed.get_2d(&1, \"one\"), Some(\"foo\"));"] # [doc = " assert_eq!(borrowed.get_2d(&1, \"two\"), None);"] # [doc = " assert_eq!(borrowed.get_2d(&2, \"one\"), Some(\"bar\"));"] # [doc = " assert_eq!(borrowed.get_2d(&2, \"two\"), Some(\"baz\"));"] # [doc = " assert_eq!(borrowed.get_2d(&3, \"three\"), None);"] # [doc = " ```"] pub fn get_2d (& self , key0 : & K0 , key1 : & K1) -> Option < & 'a V :: GetType > { self . get0 (key0) ? . get1 (key1) } }
};
}
