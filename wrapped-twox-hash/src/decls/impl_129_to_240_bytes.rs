macro_rules! impl_129_to_240_bytes {
    () => {
        # [inline] fn impl_129_to_240_bytes (secret : & Secret , seed : u64 , input : & [u8]) -> u128 { assert_input_range ! (129 ..= 240 , input . len ()) ; let input_len = input . len () . into_u64 () ; let mut acc = [input_len . wrapping_mul (PRIME64_1) , 0] ; let head = pairs_of_u64_bytes (input) ; let mut head = head . iter () ; let ss = secret . for_128 () . words_for_127_to_240_part1 () ; for (input , secret) in head . by_ref () . zip (ss) . take (4) { mix_two_chunks (& mut acc , & input [0] , & input [1] , secret , seed) ; } let mut acc = acc . map (avalanche) ; let ss = secret . for_128 () . words_for_127_to_240_part2 () ; for (input , secret) in head . zip (ss) { mix_two_chunks (& mut acc , & input [0] , & input [1] , secret , seed) ; } let (_ , tail) = input . bp_as_rchunks :: < 16 > () ; let (_ , tail) = tail . bp_as_rchunks :: < 2 > () ; let tail = tail . last () . unwrap () ; let ss = secret . for_128 () . words_for_127_to_240_part3 () ; mix_two_chunks (& mut acc , & tail [1] , & tail [0] , ss , seed . wrapping_neg ()) ; finalize_medium (acc , input_len , seed) }
    };
}

impl_129_to_240_bytes!();