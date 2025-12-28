macro_rules! deps {
    () => {
        Id!();
    };
}

macro_rules! fmt_index {
    () => {
        deps!();
        # [doc = " A helper function to show human readable fmt."] pub (crate) fn fmt_index (debug_name : & str , id : Id , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (fmt , "{debug_name}({id:?})") }
    };
}

fmt_index!();