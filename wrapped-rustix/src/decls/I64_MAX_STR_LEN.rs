macro_rules! I64_MAX_STR_LEN {
    () => {
        # [doc = " Maximum length of a formatted [`i64`]."] # [allow (dead_code)] const I64_MAX_STR_LEN : usize = "-9223372036854775808" . len () ;
    };
}

I64_MAX_STR_LEN!()