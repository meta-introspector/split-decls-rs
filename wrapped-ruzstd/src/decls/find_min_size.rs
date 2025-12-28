macro_rules! find_min_size {
    () => {
        # [doc = " Returns the minimum number of bytes needed to represent this value, as"] # [doc = " either 1, 2, 4, or 8 bytes. A value of 0 will still return one byte."] # [doc = ""] # [doc = " Used for variable length fields like `Dictionary_ID` or `Frame_Content_Size`."] pub fn find_min_size (val : u64) -> usize { if val == 0 { return 1 ; } if val >> 8 == 0 { return 1 ; } if val >> 16 == 0 { return 2 ; } if val >> 32 == 0 { return 4 ; } 8 }
    };
}

find_min_size!()