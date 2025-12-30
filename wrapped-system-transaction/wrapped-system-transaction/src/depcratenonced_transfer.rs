// Generated macro for nonced_transfer (function)
macro_rules! Depcratenonced_transfer {
() => {
// Module: crate
// Provides: {"nonced_transfer"}
// Dependencies: {}
# [doc = " Create and sign new nonced system_instruction::Transfer transaction"] pub fn nonced_transfer (from_keypair : & Keypair , to : & Pubkey , lamports : u64 , nonce_account : & Pubkey , nonce_authority : & Keypair , nonce_hash : Hash ,) -> Transaction { let from_pubkey = from_keypair . pubkey () ; let instruction = system_instruction :: transfer (& from_pubkey , to , lamports) ; let message = Message :: new_with_nonce (vec ! [instruction] , Some (& from_pubkey) , nonce_account , & nonce_authority . pubkey () ,) ; Transaction :: new (& [from_keypair , nonce_authority] , message , nonce_hash) }
};
}
