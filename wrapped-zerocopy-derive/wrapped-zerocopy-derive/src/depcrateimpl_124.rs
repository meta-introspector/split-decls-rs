// Generated macro for impl_124 (impl)
macro_rules! Depcrateimpl_124 {
() => {
// Module: crate
// Provides: {"impl_124"}
// Dependencies: {}
impl ToTokens for Trait { fn to_tokens (& self , tokens : & mut TokenStream) { let s = match self { Trait :: KnownLayout => "KnownLayout" , Trait :: Immutable => "Immutable" , Trait :: TryFromBytes => "TryFromBytes" , Trait :: FromZeros => "FromZeros" , Trait :: FromBytes => "FromBytes" , Trait :: IntoBytes => "IntoBytes" , Trait :: Unaligned => "Unaligned" , Trait :: Sized => "Sized" , Trait :: ByteHash => "ByteHash" , Trait :: ByteEq => "ByteEq" , Trait :: SplitAt => "SplitAt" , } ; let ident = Ident :: new (s , Span :: call_site ()) ; tokens . extend (core :: iter :: once (TokenTree :: Ident (ident))) ; } }
};
}
