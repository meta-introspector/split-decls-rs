macro_rules! deps {
    () => {
        Read!();
        FrameHeaderError!();
        FrameHeader!();
    };
}

macro_rules! impl_151 {
    () => {
        deps!();
        impl FrameHeader { # [doc = " Read the size of the window from the header or the total frame content size,"] # [doc = " whichever is defined, returning the size in bytes."] pub fn window_size (& self) -> Result < u64 , FrameHeaderError > { if self . descriptor . single_segment_flag () { Ok (self . frame_content_size ()) } else { let exp = self . window_descriptor >> 3 ; let mantissa = self . window_descriptor & 0x7 ; let window_log = 10 + u64 :: from (exp) ; let window_base = 1 << window_log ; let window_add = (window_base / 8) * u64 :: from (mantissa) ; let window_size = window_base + window_add ; if window_size >= MIN_WINDOW_SIZE { if window_size < MAX_WINDOW_SIZE { Ok (window_size) } else { Err (FrameHeaderError :: WindowTooBig { got : window_size }) } } else { Err (FrameHeaderError :: WindowTooSmall { got : window_size }) } } } # [doc = " The ID (if provided) of the dictionary required to decode this frame."] pub fn dictionary_id (& self) -> Option < u32 > { self . dict_id } # [doc = " Obtain the uncompressed size (in bytes) of the frame contents."] pub fn frame_content_size (& self) -> u64 { self . frame_content_size } }
    };
}

impl_151!();