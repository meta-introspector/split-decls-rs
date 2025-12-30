// Generated macro for impl_327 (impl)
macro_rules! Depcrate_de_parser_devalueimpl_327 {
() => {
// Module: crate::de::parser::devalue
// Provides: {"impl_327"}
// Dependencies: {}
impl Index for str { fn index < 'r , 'i > (& self , val : & 'r DeValue < 'i >) -> Option < & 'r Spanned < DeValue < 'i > > > { match * val { DeValue :: Table (ref a) => a . get (self) , _ => None , } } }
};
}
