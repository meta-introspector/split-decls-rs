macro_rules! xor {
    () => {
        # [doc = " XORs a slice in place with another slice."] # [inline] fn xor (dest : & mut [u8] , src : & [u8]) { assert_eq ! (dest . len () , src . len ()) ; for (lhs , rhs) in dest . iter_mut () . zip (src . iter ()) { * lhs ^= * rhs ; } }
    };
}

xor!()