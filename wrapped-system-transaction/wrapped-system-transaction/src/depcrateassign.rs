// Generated macro for assign (function)
macro_rules! Depcrateassign {
() => {
// Module: crate
// Provides: {"assign"}
// Dependencies: {}
# [doc = " Create and sign new system_instruction::Assign transaction"] pub fn assign (from_keypair : & Keypair , recent_blockhash : Hash , program_id : & Pubkey) -> Transaction { let from_pubkey = from_keypair . pubkey () ; let instruction = system_instruction :: assign (& from_pubkey , program_id) ; let message = Message :: new (& [instruction] , Some (& from_pubkey)) ; Transaction :: new (& [from_keypair] , message , recent_blockhash) }
};
}
