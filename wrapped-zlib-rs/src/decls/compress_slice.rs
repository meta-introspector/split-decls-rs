macro_rules! deps {
    () => {
        ReturnCode!();
        DeflateConfig!();
    };
}

macro_rules! compress_slice {
    () => {
        deps!();
        pub fn compress_slice < 'a > (output : & 'a mut [u8] , input : & [u8] , config : DeflateConfig ,) -> (& 'a mut [u8] , ReturnCode) { let output_uninit = unsafe { core :: slice :: from_raw_parts_mut (output . as_mut_ptr () as * mut MaybeUninit < u8 > , output . len ()) } ; compress (output_uninit , input , config) }
    };
}

compress_slice!()