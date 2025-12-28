macro_rules! deps {
    () => {
        XzEncoder!();
    };
}

macro_rules! impl_32 {
    () => {
        deps!();
        impl < R > XzEncoder < R > { # [doc = " Acquires a reference to the underlying stream"] pub fn get_ref (& self) -> & R { & self . obj } # [doc = " Acquires a mutable reference to the underlying stream"] # [doc = ""] # [doc = " Note that mutation of the stream may result in surprising results if"] # [doc = " this encoder is continued to be used."] pub fn get_mut (& mut self) -> & mut R { & mut self . obj } # [doc = " Consumes this encoder, returning the underlying reader."] pub fn into_inner (self) -> R { self . obj } # [doc = " Returns the number of bytes produced by the compressor"] # [doc = " (e.g. the number of bytes read from this stream)"] # [doc = ""] # [doc = " Note that, due to buffering, this only bears any relation to"] # [doc = " total_in() when the compressor chooses to flush its data"] # [doc = " (unfortunately, this won't happen in general at the end of the"] # [doc = " stream, because the compressor doesn't know if there's more data"] # [doc = " to come).  At that point, `total_out() / total_in()` would be"] # [doc = " the compression ratio."] pub fn total_out (& self) -> u64 { self . data . total_out () } # [doc = " Returns the number of bytes consumed by the compressor"] # [doc = " (e.g. the number of bytes read from the underlying stream)"] pub fn total_in (& self) -> u64 { self . data . total_in () } }
    };
}

impl_32!();