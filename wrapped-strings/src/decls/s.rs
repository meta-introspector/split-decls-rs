macro_rules! deps {
    () => {
        PCSTR!();
    };
}

macro_rules! s {
    () => {
        deps!();
        # [doc = " A literal UTF-8 string with a trailing null terminator."] # [macro_export] macro_rules ! s { ($ s : literal) => { $ crate :: PCSTR :: from_raw (:: core :: concat ! ($ s , '\0') . as_ptr ()) } ; }
    };
}

s!()