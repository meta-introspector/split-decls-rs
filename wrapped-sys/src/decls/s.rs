macro_rules! s {
    () => {
        # [doc = " A literal UTF-8 string with a trailing null terminator."] # [macro_export] macro_rules ! s { ($ s : literal) => { :: core :: concat ! ($ s , '\0') . as_ptr () } ; }
    };
}

s!()