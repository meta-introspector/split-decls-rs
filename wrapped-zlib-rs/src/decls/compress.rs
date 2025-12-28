macro_rules! deps {
    () => {
        ReturnCode!();
        DeflateConfig!();
        DeflateFlush!();
    };
}

macro_rules! compress {
    () => {
        deps!();
        pub fn compress < 'a > (output : & 'a mut [MaybeUninit < u8 >] , input : & [u8] , config : DeflateConfig ,) -> (& 'a mut [u8] , ReturnCode) { compress_with_flush (output , input , config , DeflateFlush :: Finish) }
    };
}

compress!();