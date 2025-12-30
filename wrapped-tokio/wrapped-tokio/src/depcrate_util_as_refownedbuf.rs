// Generated macro for OwnedBuf (enum)
macro_rules! Depcrate_util_as_refOwnedBuf {
() => {
// Module: crate::util::as_ref
// Provides: {"OwnedBuf"}
// Dependencies: {}
# [derive (Debug)] pub (crate) enum OwnedBuf { Vec (Vec < u8 >) , # [cfg (feature = "io-util")] Bytes (bytes :: Bytes) , }
};
}
