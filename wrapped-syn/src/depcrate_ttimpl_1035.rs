// Generated macro for impl_1035 (impl)
macro_rules! Depcrate_ttimpl_1035 {
() => {
// Module: crate::tt
// Provides: {"impl_1035"}
// Dependencies: {}
impl < 'a > Hash for TokenTreeHelper < 'a > { fn hash < H : Hasher > (& self , h : & mut H) { match self . 0 { TokenTree :: Group (g) => { 0u8 . hash (h) ; match g . delimiter () { Delimiter :: Parenthesis => 0u8 . hash (h) , Delimiter :: Brace => 1u8 . hash (h) , Delimiter :: Bracket => 2u8 . hash (h) , Delimiter :: None => 3u8 . hash (h) , } for item in g . stream () { TokenTreeHelper (& item) . hash (h) ; } 0xFFu8 . hash (h) ; } TokenTree :: Punct (op) => { 1u8 . hash (h) ; op . as_char () . hash (h) ; match op . spacing () { Spacing :: Alone => 0u8 . hash (h) , Spacing :: Joint => 1u8 . hash (h) , } } TokenTree :: Literal (lit) => (2u8 , lit . to_string ()) . hash (h) , TokenTree :: Ident (word) => (3u8 , word) . hash (h) , } } }
};
}
