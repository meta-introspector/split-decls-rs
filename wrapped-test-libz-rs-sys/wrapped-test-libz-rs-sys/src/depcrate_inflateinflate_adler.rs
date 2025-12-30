// Generated macro for inflate_adler (function)
macro_rules! Depcrate_inflateinflate_adler {
() => {
// Module: crate::inflate
// Provides: {"inflate_adler"}
// Dependencies: {}
# [test] fn inflate_adler () { use libz_rs_sys :: * ; const ORIGINAL : & str = "The quick brown fox jumped over the lazy dog\0" ; const COMPRESSED : [u8 ; 52] = [0x78 , 0x9c , 0x0b , 0xc9 , 0x48 , 0x55 , 0x28 , 0x2c , 0xcd , 0x4c , 0xce , 0x56 , 0x48 , 0x2a , 0xca , 0x2f , 0xcf , 0x53 , 0x48 , 0xcb , 0xaf , 0x50 , 0xc8 , 0x2a , 0xcd , 0x2d , 0x48 , 0x4d , 0x51 , 0xc8 , 0x2f , 0x4b , 0x2d , 0x52 , 0x28 , 0xc9 , 0x48 , 0x55 , 0xc8 , 0x49 , 0xac , 0xaa , 0x54 , 0x48 , 0xc9 , 0x4f , 0x07 , 0x00 , 0x6b , 0x93 , 0x10 , 0x30 ,] ; let mut stream = z_stream { next_in : std :: ptr :: null_mut () , avail_in : 0 , total_in : 0 , next_out : std :: ptr :: null_mut () , avail_out : 0 , total_out : 0 , msg : std :: ptr :: null_mut () , state : std :: ptr :: null_mut () , zalloc : None , zfree : None , opaque : std :: ptr :: null_mut () , data_type : 0 , adler : 0 , reserved : 0 , } ; let err = unsafe { inflateInit2_ (& mut stream , 32 + MAX_WBITS , VERSION , STREAM_SIZE) } ; assert_eq ! (err , Z_OK) ; let mut uncompressed = [0u8 ; 1024] ; stream . next_in = COMPRESSED . as_ptr () as * mut u8 ; stream . avail_in = COMPRESSED . len () as _ ; stream . next_out = uncompressed . as_mut_ptr () ; stream . avail_out = uncompressed . len () as _ ; let err = unsafe { inflate (& mut stream , Z_NO_FLUSH) } ; assert_eq ! (err , Z_STREAM_END) ; assert_eq ! (stream . adler , 0x6b931030) ; let err = unsafe { inflateEnd (& mut stream) } ; assert_eq ! (err , Z_OK) ; let length = Ord :: min (stream . total_out as usize , ORIGINAL . len ()) ; assert_eq ! (& uncompressed [.. length] , & ORIGINAL . as_bytes () [.. length]) }
};
}
