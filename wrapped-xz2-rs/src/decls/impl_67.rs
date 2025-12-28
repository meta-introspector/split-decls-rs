macro_rules! deps {
    () => {
        Status!();
        Stream!();
        XzDecoder!();
        Action!();
        Error!();
    };
}

macro_rules! impl_67 {
    () => {
        deps!();
        impl < W : Write > XzDecoder < W > { # [doc = " Creates a new decoding stream which will decode into `obj` one xz stream"] # [doc = " from the input written to it."] pub fn new (obj : W) -> XzDecoder < W > { let stream = Stream :: new_stream_decoder (u64 :: max_value () , 0) . unwrap () ; XzDecoder :: new_stream (obj , stream) } # [doc = " Creates a new decoding stream which will decode into `obj` all the xz streams"] # [doc = " from the input written to it."] pub fn new_multi_decoder (obj : W) -> XzDecoder < W > { let stream = Stream :: new_stream_decoder (u64 :: max_value () , lzma_sys :: LZMA_CONCATENATED) . unwrap () ; XzDecoder :: new_stream (obj , stream) } # [doc = " Creates a new decoding stream which will decode all input written to it"] # [doc = " into `obj`."] # [doc = ""] # [doc = " A custom `stream` can be specified to configure what format this decoder"] # [doc = " will recognize or configure other various decoding options."] pub fn new_stream (obj : W , stream : Stream) -> XzDecoder < W > { XzDecoder { data : stream , obj : Some (obj) , buf : Vec :: with_capacity (32 * 1024) , } } # [doc = " Acquires a reference to the underlying writer."] pub fn get_ref (& self) -> & W { self . obj . as_ref () . unwrap () } # [doc = " Acquires a mutable reference to the underlying writer."] # [doc = ""] # [doc = " Note that mutating the output/input state of the stream may corrupt this"] # [doc = " object, so care must be taken when using this method."] pub fn get_mut (& mut self) -> & mut W { self . obj . as_mut () . unwrap () } fn dump (& mut self) -> io :: Result < () > { if self . buf . len () > 0 { self . obj . as_mut () . unwrap () . write_all (& self . buf) ? ; self . buf . truncate (0) ; } Ok (()) } fn try_finish (& mut self) -> io :: Result < () > { loop { self . dump () ? ; let res = self . data . process_vec (& [] , & mut self . buf , Action :: Finish) ? ; if self . buf . is_empty () && res == Status :: MemNeeded { let msg = "xz compressed stream is truncated or otherwise corrupt" ; return Err (io :: Error :: new (io :: ErrorKind :: UnexpectedEof , msg)) ; } if res == Status :: StreamEnd { break ; } } self . dump () } # [doc = " Unwrap the underlying writer, finishing the compression stream."] pub fn finish (& mut self) -> io :: Result < W > { self . try_finish () ? ; Ok (self . obj . take () . unwrap ()) } # [doc = " Returns the number of bytes produced by the decompressor"] # [doc = ""] # [doc = " Note that, due to buffering, this only bears any relation to"] # [doc = " `total_in()` after a call to `flush()`.  At that point,"] # [doc = " `total_in() / total_out()` is the compression ratio."] pub fn total_out (& self) -> u64 { self . data . total_out () } # [doc = " Returns the number of bytes consumed by the decompressor"] # [doc = " (e.g. the number of bytes written to this stream.)"] pub fn total_in (& self) -> u64 { self . data . total_in () } }
    };
}

impl_67!()