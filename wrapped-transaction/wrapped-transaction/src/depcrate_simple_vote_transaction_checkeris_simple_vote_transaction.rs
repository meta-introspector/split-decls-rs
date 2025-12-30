// Generated macro for is_simple_vote_transaction (function)
macro_rules! Depcrate_simple_vote_transaction_checkeris_simple_vote_transaction {
() => {
// Module: crate::simple_vote_transaction_checker
// Provides: {"is_simple_vote_transaction"}
// Dependencies: {}
# [doc = " Simple vote transaction meets these conditions:"] # [doc = " 1. has 1 or 2 signatures;"] # [doc = " 2. is legacy message;"] # [doc = " 3. has only one instruction;"] # [doc = " 4. which must be Vote instruction;"] pub fn is_simple_vote_transaction (sanitized_versioned_transaction : & SanitizedVersionedTransaction ,) -> bool { let is_legacy_message = matches ! (sanitized_versioned_transaction . message . message , VersionedMessage :: Legacy (_)) ; let instruction_programs = sanitized_versioned_transaction . message . program_instructions_iter () . map (| (program_id , _ix) | program_id) ; is_simple_vote_transaction_impl (& sanitized_versioned_transaction . signatures , is_legacy_message , instruction_programs ,) }
};
}
