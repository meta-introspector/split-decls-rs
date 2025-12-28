macro_rules! ceil_log2_pow5 {
    () => {
        # [cfg_attr (feature = "no-panic" , inline)] # [allow (dead_code)] pub fn ceil_log2_pow5 (e : i32) -> i32 { log2_pow5 (e) + 1 }
    };
}

ceil_log2_pow5!()