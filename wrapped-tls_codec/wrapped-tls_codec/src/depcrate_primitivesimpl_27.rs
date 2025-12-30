// Generated macro for impl_27 (impl)
macro_rules! Depcrate_primitivesimpl_27 {
() => {
// Module: crate::primitives
// Provides: {"impl_27"}
// Dependencies: {}
impl < T : Deserialize > Deserialize for Option < T > { # [cfg (feature = "std")] # [inline] fn tls_deserialize < R : Read > (bytes : & mut R) -> Result < Self , Error > { let mut some_or_none = [0u8 ; 1] ; bytes . read_exact (& mut some_or_none) ? ; match some_or_none [0] { 0 => Ok (None) , 1 => { let element = T :: tls_deserialize (bytes) ? ; Ok (Some (element)) } _ => Err (Error :: DecodingError (format ! ("Trying to decode Option<T> with {} for option. It must be 0 for None and 1 for Some." , some_or_none [0]))) , } } }
};
}
