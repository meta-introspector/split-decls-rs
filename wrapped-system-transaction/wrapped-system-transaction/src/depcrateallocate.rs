// Generated macro for allocate (function)
macro_rules! Depcrateallocate {
() => {
// Module: crate
// Provides: {"allocate"}
// Dependencies: {}
# [doc = " Create and sign new SystemInstruction::Allocate transaction"] pub fn allocate (payer_keypair : & Keypair , account_keypair : & Keypair , recent_blockhash : Hash , space : u64 ,) -> Transaction { let payer_pubkey = payer_keypair . pubkey () ; let account_pubkey = account_keypair . pubkey () ; let instruction = system_instruction :: allocate (& account_pubkey , space) ; let message = Message :: new (& [instruction] , Some (& payer_pubkey)) ; Transaction :: new (& [payer_keypair , account_keypair] , message , recent_blockhash) }
};
}
