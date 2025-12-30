// Generated macro for is_simple_vote_transaction_impl (function)
macro_rules! Depcrate_simple_vote_transaction_checkeris_simple_vote_transaction_impl {
() => {
// Module: crate::simple_vote_transaction_checker
// Provides: {"is_simple_vote_transaction_impl"}
// Dependencies: {}
# [doc = " Simple vote transaction meets these conditions:"] # [doc = " 1. has 1 or 2 signatures;"] # [doc = " 2. is legacy message;"] # [doc = " 3. has only one instruction;"] # [doc = " 4. which must be Vote instruction;"] # [inline] pub fn is_simple_vote_transaction_impl < 'a > (signatures : & [Signature] , is_legacy_message : bool , mut instruction_programs : impl Iterator < Item = & 'a Address > ,) -> bool { signatures . len () < 3 && is_legacy_message && instruction_programs . next () . xor (instruction_programs . next ()) . map (| program_id | program_id == & solana_sdk_ids :: vote :: ID) . unwrap_or (false) }
};
}
