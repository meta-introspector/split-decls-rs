// Generated macro for to_seq_id (function)
macro_rules! Depcrate_cfi_typeid_itanium_cxx_abi_encodeto_seq_id {
() => {
// Module: crate::cfi::typeid::itanium_cxx_abi::encode
// Provides: {"to_seq_id"}
// Dependencies: {}
# [doc = " Converts a number to a sequence number (see"] # [doc = " <https://itanium-cxx-abi.github.io/cxx-abi/abi.html#mangle.seq-id>)."] fn to_seq_id (num : usize) -> String { if let Some (num) = num . checked_sub (1) { (num as u64) . to_base (CASE_INSENSITIVE) . to_uppercase () } else { "" . to_string () } }
};
}
