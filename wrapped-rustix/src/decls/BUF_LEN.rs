macro_rules! BUF_LEN {
    () => {
        # [doc = " Enough to hold an {u,i}64 and NUL terminator."] const BUF_LEN : usize = U64_MAX_STR_LEN + 1 ;
    };
}

BUF_LEN!();