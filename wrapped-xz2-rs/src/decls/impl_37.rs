macro_rules! deps {
    () => {
        Stream!();
        XzDecoder!();
    };
}

macro_rules! impl_37 {
    () => {
        deps!();
        impl < R : BufRead > XzDecoder < R > { # [doc = " Creates a new decoder which will decompress data read from the given"] # [doc = " stream."] pub fn new (r : R) -> XzDecoder < R > { let stream = Stream :: new_stream_decoder (u64 :: max_value () , 0) . unwrap () ; XzDecoder :: new_stream (r , stream) } # [doc = " Creates a new decoder which will decompress data read from the given"] # [doc = " input. All the concatenated xz streams from input will be consumed."] pub fn new_multi_decoder (r : R) -> XzDecoder < R > { let stream = Stream :: new_auto_decoder (u64 :: max_value () , lzma_sys :: LZMA_CONCATENATED) . unwrap () ; XzDecoder :: new_stream (r , stream) } # [doc = " Creates a new decoder with a custom `Stream`."] # [doc = ""] # [doc = " The `Stream` can be pre-configured for various checks, different"] # [doc = " decompression options/tuning, etc."] pub fn new_stream (r : R , stream : Stream) -> XzDecoder < R > { XzDecoder { obj : r , data : stream , } } }
    };
}

impl_37!();