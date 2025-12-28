macro_rules! deps {
    () => {
        StaticTreeDesc!();
        Value!();
    };
}

macro_rules! encode_len {
    () => {
        deps!();
        # [inline] const fn encode_len (ltree : & [Value] , lc : u8) -> (u64 , usize) { let mut lc = lc as usize ; let code = self :: trees_tbl :: LENGTH_CODE [lc] as usize ; let c = code + LITERALS + 1 ; assert ! (c < L_CODES , "bad l_code") ; let lnode = ltree [c] ; let mut match_bits : u64 = lnode . code () as u64 ; let mut match_bits_len = lnode . len () as usize ; let extra = StaticTreeDesc :: EXTRA_LBITS [code] as usize ; if extra != 0 { lc -= self :: trees_tbl :: BASE_LENGTH [code] as usize ; match_bits |= (lc as u64) << match_bits_len ; match_bits_len += extra ; } (match_bits , match_bits_len) }
    };
}

encode_len!()