macro_rules! deps {
    () => {
        XzDecoder!();
    };
}

macro_rules! impl_38 {
    () => {
        deps!();
        impl < R > XzDecoder < R > { # [doc = " Acquires a reference to the underlying stream"] pub fn get_ref (& self) -> & R { & self . obj } # [doc = " Acquires a mutable reference to the underlying stream"] # [doc = ""] # [doc = " Note that mutation of the stream may result in surprising results if"] # [doc = " this encoder is continued to be used."] pub fn get_mut (& mut self) -> & mut R { & mut self . obj } # [doc = " Consumes this decoder, returning the underlying reader."] pub fn into_inner (self) -> R { self . obj } # [doc = " Returns the number of bytes that the decompressor has consumed."] # [doc = ""] # [doc = " Note that this will likely be smaller than what the decompressor"] # [doc = " actually read from the underlying stream due to buffering."] pub fn total_in (& self) -> u64 { self . data . total_in () } # [doc = " Returns the number of bytes that the decompressor has produced."] pub fn total_out (& self) -> u64 { self . data . total_out () } }
    };
}

impl_38!();