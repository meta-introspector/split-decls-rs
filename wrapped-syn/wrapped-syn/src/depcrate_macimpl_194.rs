// Generated macro for impl_194 (impl)
macro_rules! Depcrate_macimpl_194 {
() => {
// Module: crate::mac
// Provides: {"impl_194"}
// Dependencies: {}
# [cfg (feature = "extra-traits")] impl :: std :: hash :: Hash for TokenTree { fn hash < H : :: std :: hash :: Hasher > (& self , h : & mut H) { use proc_macro2 :: Spacing ; match self . 0 . kind { TokenNode :: Group (delim , ref stream) => { 0u8 . hash (h) ; match delim { Delimiter :: Parenthesis => 0u8 . hash (h) , Delimiter :: Brace => 1u8 . hash (h) , Delimiter :: Bracket => 2u8 . hash (h) , Delimiter :: None => 3u8 . hash (h) , } for item in stream . clone () . into_iter () { TokenTree (item) . hash (h) ; } 0xffu8 . hash (h) ; } TokenNode :: Op (op , kind) => { 1u8 . hash (h) ; op . hash (h) ; match kind { Spacing :: Alone => 0u8 . hash (h) , Spacing :: Joint => 1u8 . hash (h) , } } TokenNode :: Literal (ref lit) => (2u8 , lit . to_string ()) . hash (h) , TokenNode :: Term (ref word) => (3u8 , word . as_str ()) . hash (h) , } } }
};
}
