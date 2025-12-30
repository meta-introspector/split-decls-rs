// Generated macro for Context (struct)
macro_rules! Depcrate_common_stateContext {
() => {
// Module: crate::common_state
// Provides: {"Context"}
// Dependencies: {}
pub (crate) struct Context < 'a , Data > { pub (crate) common : & 'a mut CommonState , pub (crate) data : & 'a mut Data , # [doc = " Buffered plaintext. This is `Some` if any plaintext was written during handshake and `None`"] # [doc = " otherwise."] pub (crate) sendable_plaintext : Option < & 'a mut ChunkVecBuffer > , }
};
}
