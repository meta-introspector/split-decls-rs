macro_rules! deps {
    () => {
        X128!();
    };
}

macro_rules! finalize_medium {
    () => {
        deps!();
        # [inline] fn finalize_medium (acc : [u64 ; 2] , input_len : u64 , seed : u64) -> u128 { let low = acc [0] . wrapping_add (acc [1]) ; let high = acc [0] . wrapping_mul (PRIME64_1) . wrapping_add (acc [1] . wrapping_mul (PRIME64_4)) . wrapping_add ((input_len . wrapping_sub (seed)) . wrapping_mul (PRIME64_2)) ; let low = avalanche (low) ; let high = avalanche (high) . wrapping_neg () ; X128 { low , high } . into () }
    };
}

finalize_medium!();