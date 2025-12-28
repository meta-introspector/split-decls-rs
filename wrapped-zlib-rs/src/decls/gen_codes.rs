macro_rules! deps {
    () => {
        Value!();
    };
}

macro_rules! gen_codes {
    () => {
        deps!();
        fn gen_codes (tree : & mut [Value] , max_code : usize , bl_count : & [u16]) { let mut next_code = [0 ; MAX_BITS + 1] ; let mut code = 0 ; for bits in 1 ..= MAX_BITS { code = (code + bl_count [bits - 1]) << 1 ; next_code [bits] = code ; } assert ! (code + bl_count [MAX_BITS] - 1 == (1 << MAX_BITS) - 1 , "inconsistent bit counts") ; trace ! ("\ngen_codes: max_code {max_code} ") ; for n in 0 ..= max_code { let len = tree [n] . len () ; if len == 0 { continue ; } assert ! ((1 ..= 15) . contains (& len) , "code length must be 1-15") ; * tree [n] . code_mut () = next_code [len as usize] . reverse_bits () >> (16 - len) ; next_code [len as usize] += 1 ; if tree != self :: trees_tbl :: STATIC_LTREE . as_slice () { trace ! ("\nn {:>3} {} l {:>2} c {:>4x} ({:x}) " , n , if isgraph (n as u8) { char :: from_u32 (n as u32) . unwrap () } else { ' ' } , len , tree [n] . code () , next_code [len as usize] - 1) ; } } }
    };
}

gen_codes!();