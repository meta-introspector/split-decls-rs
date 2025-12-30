// Generated macro for impl_613 (impl)
macro_rules! Depcrate_common_stateimpl_613 {
() => {
// Module: crate::common_state
// Provides: {"impl_613"}
// Dependencies: {}
# [cfg (feature = "std")] impl CommonState { # [doc = " Send plaintext application data, fragmenting and"] # [doc = " encrypting it as it goes out."] # [doc = ""] # [doc = " If internal buffers are too small, this function will not accept"] # [doc = " all the data."] pub (crate) fn buffer_plaintext (& mut self , payload : OutboundChunks < '_ > , sendable_plaintext : & mut ChunkVecBuffer ,) -> usize { self . perhaps_write_key_update () ; self . send_plain (payload , Limit :: Yes , sendable_plaintext) } pub (crate) fn send_early_plaintext (& mut self , data : & [u8]) -> usize { debug_assert ! (self . early_traffic) ; debug_assert ! (self . record_layer . is_encrypting ()) ; if data . is_empty () { return 0 ; } self . send_appdata_encrypt (data . into () , Limit :: Yes) } # [doc = " Encrypt and send some plaintext `data`.  `limit` controls"] # [doc = " whether the per-connection buffer limits apply."] # [doc = ""] # [doc = " Returns the number of bytes written from `data`: this might"] # [doc = " be less than `data.len()` if buffer limits were exceeded."] fn send_plain (& mut self , payload : OutboundChunks < '_ > , limit : Limit , sendable_plaintext : & mut ChunkVecBuffer ,) -> usize { if ! self . may_send_application_data { let len = match limit { Limit :: Yes => sendable_plaintext . append_limited_copy (payload) , Limit :: No => sendable_plaintext . append (payload . to_vec ()) , } ; return len ; } self . send_plain_non_buffering (payload , limit) } }
};
}
