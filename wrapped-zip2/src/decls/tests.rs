macro_rules! tests {
    () => {
        # [cfg (test)] mod tests { use crate :: legacy :: shrink :: hwunshrink ; const LZW_FIG5 : & [u8 ; 17] = b"ababcbababaaaaaaa" ; const LZW_FIG5_SHRUNK : [u8 ; 12] = [0x61 , 0xc4 , 0x04 , 0x1c , 0x23 , 0xb0 , 0x60 , 0x98 , 0x83 , 0x08 , 0xc3 , 0x00 ,] ; # [test] fn test_unshrink_lzw_fig5 () { let mut dst = Vec :: with_capacity (LZW_FIG5 . len ()) ; hwunshrink (& LZW_FIG5_SHRUNK , LZW_FIG5 . len () , & mut dst) . unwrap () ; assert_eq ! (dst . as_slice () , LZW_FIG5) ; } }
    };
}

tests!();