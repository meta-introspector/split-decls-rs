// Generated macro for convert_in_scheme_handle_block (function)
macro_rules! Depcrate_schemeconvert_in_scheme_handle_block {
() => {
// Module: crate::scheme
// Provides: {"convert_in_scheme_handle_block"}
// Dependencies: {}
pub (crate) fn convert_in_scheme_handle_block (_ : & Packet , result : Result < Option < OpenResult > > ,) -> Result < Option < usize > > { match result { Ok (Some (OpenResult :: ThisScheme { number })) => Ok (Some (number)) , Ok (Some (OpenResult :: OtherScheme { .. })) => Err (Error :: new (EOPNOTSUPP)) , Ok (None) => Ok (None) , Err (err) => Err (err) , } }
};
}
