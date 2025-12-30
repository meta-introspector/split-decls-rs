// Generated macro for convert_in_scheme_handle (function)
macro_rules! Depcrate_schemeconvert_in_scheme_handle {
() => {
// Module: crate::scheme
// Provides: {"convert_in_scheme_handle"}
// Dependencies: {}
pub (crate) fn convert_in_scheme_handle (packet : & mut Packet , result : Result < OpenResult > ,) -> Result < usize > { match result { Ok (OpenResult :: ThisScheme { number }) => Ok (number) , Ok (OpenResult :: OtherScheme { fd }) => { packet . b = SKMSG_FRETURNFD ; packet . c = fd ; Err (Error :: new (ESKMSG)) } Err (err) => Err (err) , } }
};
}
