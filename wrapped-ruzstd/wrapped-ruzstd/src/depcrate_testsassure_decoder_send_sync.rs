// Generated macro for assure_decoder_send_sync (function)
macro_rules! Depcrate_testsassure_decoder_send_sync {
() => {
// Module: crate::tests
// Provides: {"assure_decoder_send_sync"}
// Dependencies: {}
# [cfg (all (test , feature = "std"))] # [allow (dead_code)] fn assure_decoder_send_sync () { use crate :: decoding :: FrameDecoder ; let decoder = FrameDecoder :: new () ; std :: thread :: spawn (move | | { drop (decoder) ; }) ; }
};
}
