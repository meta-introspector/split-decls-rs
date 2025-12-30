// Generated macro for read_option_bls_public_key_compressed (function)
macro_rules! Depcrate_state_vote_state_deserializeread_option_bls_public_key_compressed {
() => {
// Module: crate::state::vote_state_deserialize
// Provides: {"read_option_bls_public_key_compressed"}
// Dependencies: {}
fn read_option_bls_public_key_compressed < T : AsRef < [u8] > > (cursor : & mut Cursor < T > ,) -> Result < Option < [u8 ; BLS_PUBLIC_KEY_COMPRESSED_SIZE] > , InstructionError > { let variant = read_u8 (cursor) ? ; match variant { 0 => Ok (None) , 1 => { let mut buf = [0 ; BLS_PUBLIC_KEY_COMPRESSED_SIZE] ; cursor . read_exact (& mut buf) . map_err (| _ | InstructionError :: InvalidAccountData) ? ; Ok (Some (buf)) } _ => Err (InstructionError :: InvalidAccountData) , } }
};
}
