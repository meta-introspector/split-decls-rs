// Generated macro for impl_326 (impl)
macro_rules! Depcrate_de_parser_devalueimpl_326 {
() => {
// Module: crate::de::parser::devalue
// Provides: {"impl_326"}
// Dependencies: {}
impl Index for usize { fn index < 'r , 'i > (& self , val : & 'r DeValue < 'i >) -> Option < & 'r Spanned < DeValue < 'i > > > { match * val { DeValue :: Array (ref a) => a . get (* self) , _ => None , } } }
};
}
