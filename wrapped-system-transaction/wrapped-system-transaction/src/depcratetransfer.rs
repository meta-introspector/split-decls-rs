// Generated macro for transfer (function)
macro_rules! Depcratetransfer {
() => {
// Module: crate
// Provides: {"transfer"}
// Dependencies: {}
# [doc = " Create and sign new system_instruction::Transfer transaction"] pub fn transfer (from_keypair : & Keypair , to : & Pubkey , lamports : u64 , recent_blockhash : Hash ,) -> Transaction { let from_pubkey = from_keypair . pubkey () ; let instruction = system_instruction :: transfer (& from_pubkey , to , lamports) ; let message = Message :: new (& [instruction] , Some (& from_pubkey)) ; Transaction :: new (& [from_keypair] , message , recent_blockhash) }
};
}
