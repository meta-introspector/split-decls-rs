// Generated macro for read_pubkey_into (function)
macro_rules! Depcrate_cursorread_pubkey_into {
() => {
// Module: crate::cursor
// Provides: {"read_pubkey_into"}
// Dependencies: {}
pub fn read_pubkey_into (cursor : & mut Cursor < & [u8] > , pubkey : * mut Pubkey ,) -> Result < () , InstructionError > { match cursor . fill_buf () { Ok (buf) if buf . len () >= PUBKEY_BYTES => { unsafe { ptr :: copy_nonoverlapping (buf . as_ptr () , pubkey as * mut u8 , PUBKEY_BYTES) ; } cursor . consume (PUBKEY_BYTES) ; } _ => return Err (InstructionError :: InvalidAccountData) , } Ok (()) }
};
}
