// Generated macro for impl_395 (impl)
macro_rules! Depcrate_content_encodingimpl_395 {
() => {
// Module: crate::content_encoding
// Provides: {"impl_395"}
// Dependencies: {}
# [cfg (any (feature = "compression-gzip" , feature = "compression-br" , feature = "compression-zstd" , feature = "compression-deflate" , feature = "fs" ,))] impl QValue { # [inline] pub (crate) fn one () -> Self { Self (1000) } fn parse (s : & str) -> Option < Self > { let mut c = s . chars () ; match c . next () { Some ('q' | 'Q') => () , _ => return None , } ; match c . next () { Some ('=') => () , _ => return None , } ; let mut value = match c . next () { Some ('0') => 0 , Some ('1') => 1000 , _ => return None , } ; match c . next () { Some ('.') => () , None => return Some (Self (value)) , _ => return None , } ; let mut factor = 100 ; loop { match c . next () { Some (n @ '0' ..= '9') => { if factor < 1 { return None ; } value += factor * (n as u16 - '0' as u16) ; } None => { return if value <= 1000 { Some (Self (value)) } else { None } ; } _ => return None , } ; factor /= 10 ; } } }
};
}
