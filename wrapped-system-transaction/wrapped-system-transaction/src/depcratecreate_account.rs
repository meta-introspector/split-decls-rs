// Generated macro for create_account (function)
macro_rules! Depcratecreate_account {
() => {
// Module: crate
// Provides: {"create_account"}
// Dependencies: {}
# [doc = " Create and sign new SystemInstruction::CreateAccount transaction"] pub fn create_account (from_keypair : & Keypair , to_keypair : & Keypair , recent_blockhash : Hash , lamports : u64 , space : u64 , program_id : & Pubkey ,) -> Transaction { let from_pubkey = from_keypair . pubkey () ; let to_pubkey = to_keypair . pubkey () ; let instruction = system_instruction :: create_account (& from_pubkey , & to_pubkey , lamports , space , program_id) ; let message = Message :: new (& [instruction] , Some (& from_pubkey)) ; Transaction :: new (& [from_keypair , to_keypair] , message , recent_blockhash) }
};
}
