// Generated macro for skippable_frame (function)
macro_rules! Depcrate_testsskippable_frame {
() => {
// Module: crate::tests
// Provides: {"skippable_frame"}
// Dependencies: {}
# [test] fn skippable_frame () { use crate :: decoding :: errors ; use crate :: decoding :: frame ; let mut content = vec ! [] ; content . extend_from_slice (& 0x184D2A50u32 . to_le_bytes ()) ; content . extend_from_slice (& 300u32 . to_le_bytes ()) ; assert_eq ! (8 , content . len ()) ; let err = frame :: read_frame_header (content . as_slice ()) ; assert ! (matches ! (err , Err (errors :: ReadFrameHeaderError :: SkipFrame { magic_number : 0x184D2A50u32 , length : 300 }))) ; content . clear () ; content . extend_from_slice (& 0x184D2A5Fu32 . to_le_bytes ()) ; content . extend_from_slice (& 0xFFFFFFFFu32 . to_le_bytes ()) ; assert_eq ! (8 , content . len ()) ; let err = frame :: read_frame_header (content . as_slice ()) ; assert ! (matches ! (err , Err (errors :: ReadFrameHeaderError :: SkipFrame { magic_number : 0x184D2A5Fu32 , length : 0xFFFFFFFF }))) ; }
};
}
