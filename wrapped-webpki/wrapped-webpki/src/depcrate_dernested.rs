// Generated macro for nested (function)
macro_rules! Depcrate_dernested {
() => {
// Module: crate::der
// Provides: {"nested"}
// Dependencies: {}
pub (crate) fn nested < 'a , R > (input : & mut untrusted :: Reader < 'a > , tag : Tag , error : Error , decoder : impl FnOnce (& mut untrusted :: Reader < 'a >) -> Result < R , Error > ,) -> Result < R , Error > { nested_limited (input , tag , error , decoder , TWO_BYTE_DER_SIZE) }
};
}
