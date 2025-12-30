// Generated macro for bit_string_flags (function)
macro_rules! Depcrate_derbit_string_flags {
() => {
// Module: crate::der
// Provides: {"bit_string_flags"}
// Dependencies: {}
pub (crate) fn bit_string_flags (input : untrusted :: Input < '_ >) -> Result < BitStringFlags < '_ > , Error > { input . read_all (Error :: BadDer , | bit_string | { let padding_bits = bit_string . read_byte () . map_err (| _ | Error :: BadDer) ? ; let raw_bits = bit_string . read_bytes_to_end () . as_slice_less_safe () ; if padding_bits > 7 || (raw_bits . is_empty () && padding_bits != 0) { return Err (Error :: BadDer) ; } let last_byte = raw_bits [raw_bits . len () - 1] ; let padding_mask = (1 << padding_bits) - 1 ; match padding_bits > 0 && (last_byte & padding_mask) != 0 { true => Err (Error :: BadDer) , false => Ok (BitStringFlags { raw_bits }) , } }) }
};
}
