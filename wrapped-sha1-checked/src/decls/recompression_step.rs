macro_rules! deps {
    () => {
        Testt!();
    };
}

macro_rules! recompression_step {
    () => {
        deps!();
        fn recompression_step (step : Testt , ihvin : & mut [u32 ; 5] , ihvout : & mut [u32 ; 5] , me2 : & [u32 ; 80] , state : & [u32 ; 5] ,) { match step { Testt :: T58 => { recompress_fast_58 (ihvin , ihvout , me2 , state) ; } Testt :: T65 => { recompress_fast_65 (ihvin , ihvout , me2 , state) ; } } }
    };
}

recompression_step!();