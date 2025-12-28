macro_rules! decimal_length17 {
    () => {
        # [cfg_attr (feature = "no-panic" , inline)] pub fn decimal_length17 (v : u64) -> u32 { debug_assert ! (v < 100000000000000000) ; if v >= 10000000000000000 { 17 } else if v >= 1000000000000000 { 16 } else if v >= 100000000000000 { 15 } else if v >= 10000000000000 { 14 } else if v >= 1000000000000 { 13 } else if v >= 100000000000 { 12 } else if v >= 10000000000 { 11 } else if v >= 1000000000 { 10 } else if v >= 100000000 { 9 } else if v >= 10000000 { 8 } else if v >= 1000000 { 7 } else if v >= 100000 { 6 } else if v >= 10000 { 5 } else if v >= 1000 { 4 } else if v >= 100 { 3 } else if v >= 10 { 2 } else { 1 } }
    };
}

decimal_length17!();