// Generated macro for nested_of_mut (function)
macro_rules! Depcrate_dernested_of_mut {
() => {
// Module: crate::der
// Provides: {"nested_of_mut"}
// Dependencies: {}
pub (crate) fn nested_of_mut < 'a > (input : & mut untrusted :: Reader < 'a > , outer_tag : Tag , inner_tag : Tag , error : Error , allow_empty : bool , mut decoder : impl FnMut (& mut untrusted :: Reader < 'a >) -> Result < () , Error > ,) -> Result < () , Error > { nested (input , outer_tag , error . clone () , | outer | { if allow_empty && outer . at_end () { return Ok (()) ; } loop { nested (outer , inner_tag , error . clone () , | inner | decoder (inner)) ? ; if outer . at_end () { break ; } } Ok (()) }) }
};
}
