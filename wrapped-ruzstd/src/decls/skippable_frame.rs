macro_rules! deps {
    () => {
        ReadFrameHeaderError!();
    };
}

macro_rules! skippable_frame {
    () => {
        deps!();
        # [test] fn skippable_frame () { use crate :: decoding :: errors ; use crate :: decoding :: frame ; let mut content = vec ! [] ; content . extend_from_slice (& 0x184D2A50u32 . to_le_bytes ()) ; content . extend_from_slice (& 300u32 . to_le_bytes ()) ; assert_eq ! (8 , content . len ()) ; let err = frame :: read_frame_header (content . as_slice ()) ; assert ! (matches ! (err , Err (errors :: ReadFrameHeaderError :: SkipFrame { magic_number : 0x184D2A50u32 , length : 300 }))) ; content . clear () ; content . extend_from_slice (& 0x184D2A5Fu32 . to_le_bytes ()) ; content . extend_from_slice (& 0xFFFFFFFFu32 . to_le_bytes ()) ; assert_eq ! (8 , content . len ()) ; let err = frame :: read_frame_header (content . as_slice ()) ; assert ! (matches ! (err , Err (errors :: ReadFrameHeaderError :: SkipFrame { magic_number : 0x184D2A5Fu32 , length : 0xFFFFFFFF }))) ; }
    };
}

skippable_frame!()