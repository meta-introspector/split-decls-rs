// Generated macro for try_parse (function)
macro_rules! Depcrate_layer_idtry_parse {
() => {
// Module: crate::layer::id
// Provides: {"try_parse"}
// Dependencies: {}
pub (crate) const fn try_parse (input : & [u8]) -> Option < Uuid > { match (input . len () , input) { (32 , s) => parse_simple (s) , (36 , s) | (38 , [b'{' , s @ .. , b'}']) | (45 , [b'u' , b'r' , b'n' , b':' , b'u' , b'u' , b'i' , b'd' , b':' , s @ ..]) => { parse_hyphenated (s) } _ => None , } }
};
}
