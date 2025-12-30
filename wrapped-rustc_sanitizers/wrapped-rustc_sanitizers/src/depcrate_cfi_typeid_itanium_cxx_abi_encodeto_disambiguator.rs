// Generated macro for to_disambiguator (function)
macro_rules! Depcrate_cfi_typeid_itanium_cxx_abi_encodeto_disambiguator {
() => {
// Module: crate::cfi::typeid::itanium_cxx_abi::encode
// Provides: {"to_disambiguator"}
// Dependencies: {}
# [doc = " Converts a number to a disambiguator (see"] # [doc = " <https://rust-lang.github.io/rfcs/2603-rust-symbol-name-mangling-v0.html>)."] fn to_disambiguator (num : u64) -> String { if let Some (num) = num . checked_sub (1) { format ! ("s{}_" , num . to_base (ALPHANUMERIC_ONLY)) } else { "s_" . to_string () } }
};
}
