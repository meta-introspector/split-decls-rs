macro_rules! mix_two_chunks {
    () => {
        # [inline] fn mix_two_chunks (acc : & mut [u64 ; 2] , data1 : & [u8 ; 16] , data2 : & [u8 ; 16] , secret : & [[u8 ; 16] ; 2] , seed : u64 ,) { let data_words1 = to_u64s (data1) ; let data_words2 = to_u64s (data2) ; acc [0] = acc [0] . wrapping_add (mix_step (data1 , & secret [0] , seed)) ; acc [1] = acc [1] . wrapping_add (mix_step (data2 , & secret [1] , seed)) ; acc [0] ^= data_words2 [0] . wrapping_add (data_words2 [1]) ; acc [1] ^= data_words1 [0] . wrapping_add (data_words1 [1]) ; }
    };
}

mix_two_chunks!();