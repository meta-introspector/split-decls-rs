macro_rules! deps {
    () => {
        DeflateConfig!();
        DeflateFlush!();
        ReturnCode!();
    };
}

macro_rules! compress_slice_with_flush {
    () => {
        deps!();
        pub fn compress_slice_with_flush < 'a > (output : & 'a mut [u8] , input : & [u8] , config : DeflateConfig , flush : DeflateFlush ,) -> (& 'a mut [u8] , ReturnCode) { let output_uninit = unsafe { core :: slice :: from_raw_parts_mut (output . as_mut_ptr () as * mut MaybeUninit < u8 > , output . len ()) } ; compress_with_flush (output_uninit , input , config , flush) }
    };
}

compress_slice_with_flush!()