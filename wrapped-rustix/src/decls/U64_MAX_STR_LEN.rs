macro_rules! U64_MAX_STR_LEN {
    () => {
        # [doc = " Maximum length of a formatted [`u64`]."] const U64_MAX_STR_LEN : usize = "18446744073709551615" . len () ;
    };
}

U64_MAX_STR_LEN!();