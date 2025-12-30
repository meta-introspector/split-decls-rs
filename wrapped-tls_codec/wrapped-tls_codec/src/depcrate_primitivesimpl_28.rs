// Generated macro for impl_28 (impl)
macro_rules! Depcrate_primitivesimpl_28 {
() => {
// Module: crate::primitives
// Provides: {"impl_28"}
// Dependencies: {}
impl < T : DeserializeBytes > DeserializeBytes for Option < T > { # [inline] fn tls_deserialize_bytes (bytes : & [u8]) -> Result < (Self , & [u8]) , Error > { let (some_or_none , remainder) = < u8 > :: tls_deserialize_bytes (bytes) ? ; match some_or_none { 0 => Ok ((None , remainder)) , 1 => { let (element , remainder) = T :: tls_deserialize_bytes (remainder) ? ; Ok ((Some (element) , remainder)) } _ => Err (Error :: DecodingError (alloc :: format ! ("Trying to decode Option<T> with {some_or_none} for option. It must be 0 for None and 1 for Some."))) , } } }
};
}
