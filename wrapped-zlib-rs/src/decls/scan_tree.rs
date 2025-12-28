macro_rules! deps {
    () => {
        Value!();
        TreeDesc!();
    };
}

macro_rules! scan_tree {
    () => {
        deps!();
        fn scan_tree (bl_desc : & mut TreeDesc < { 2 * BL_CODES + 1 } > , tree : & mut [Value] , max_code : usize) { let mut prevlen = - 1isize ; let mut curlen : isize ; let mut nextlen = tree [0] . len () ; let mut count = 0 ; let mut max_count = 7 ; let mut min_count = 4 ; if nextlen == 0 { max_count = 138 ; min_count = 3 ; } * tree [max_code + 1] . len_mut () = 0xffff ; let bl_tree = & mut bl_desc . dyn_tree ; for n in 0 ..= max_code { curlen = nextlen as isize ; nextlen = tree [n + 1] . len () ; count += 1 ; if count < max_count && curlen == nextlen as isize { continue ; } else if count < min_count { * bl_tree [curlen as usize] . freq_mut () += count ; } else if curlen != 0 { if curlen != prevlen { * bl_tree [curlen as usize] . freq_mut () += 1 ; } * bl_tree [REP_3_6] . freq_mut () += 1 ; } else if count <= 10 { * bl_tree [REPZ_3_10] . freq_mut () += 1 ; } else { * bl_tree [REPZ_11_138] . freq_mut () += 1 ; } count = 0 ; prevlen = curlen ; if nextlen == 0 { max_count = 138 ; min_count = 3 ; } else if curlen == nextlen as isize { max_count = 6 ; min_count = 3 ; } else { max_count = 7 ; min_count = 4 ; } } }
    };
}

scan_tree!()