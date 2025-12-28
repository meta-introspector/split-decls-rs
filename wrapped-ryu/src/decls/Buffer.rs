macro_rules! Buffer {
    () => {
        # [doc = " Safe API for formatting floating point numbers to text."] # [doc = ""] # [doc = " ## Example"] # [doc = ""] # [doc = " ```"] # [doc = " let mut buffer = ryu::Buffer::new();"] # [doc = " let printed = buffer.format_finite(1.234);"] # [doc = " assert_eq!(printed, \"1.234\");"] # [doc = " ```"] pub struct Buffer { bytes : [MaybeUninit < u8 > ; 24] , }
    };
}

Buffer!()