// Generated macro for bit_string_with_no_unused_bits (function)
macro_rules! Depcrate_derbit_string_with_no_unused_bits {
() => {
// Module: crate::der
// Provides: {"bit_string_with_no_unused_bits"}
// Dependencies: {}
pub (crate) fn bit_string_with_no_unused_bits < 'a > (input : & mut untrusted :: Reader < 'a > ,) -> Result < untrusted :: Input < 'a > , Error > { nested (input , Tag :: BitString , Error :: TrailingData (DerTypeId :: BitString) , | value | { let unused_bits_at_end = value . read_byte () . map_err (| _ | Error :: BadDer) ? ; if unused_bits_at_end != 0 { return Err (Error :: BadDer) ; } Ok (value . read_bytes_to_end ()) } ,) }
};
}
