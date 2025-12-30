// Generated macro for RecordLayer (struct)
macro_rules! Depcrate_record_layerRecordLayer {
() => {
// Module: crate::record_layer
// Provides: {"RecordLayer"}
// Dependencies: {}
# [doc = " Record layer that tracks decryption and encryption keys."] pub (crate) struct RecordLayer { message_encrypter : Box < dyn MessageEncrypter > , message_decrypter : Box < dyn MessageDecrypter > , write_seq_max : u64 , write_seq : u64 , read_seq : u64 , has_decrypted : bool , encrypt_state : DirectionState , decrypt_state : DirectionState , trial_decryption_len : Option < usize > , }
};
}
