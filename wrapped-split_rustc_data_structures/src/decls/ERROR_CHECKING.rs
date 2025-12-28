macro_rules! ERROR_CHECKING {
    () => {
        # [doc = " This makes locks panic if they are already held."] # [doc = " It is only useful when you are running in a single thread"] const ERROR_CHECKING : bool = false ;
    };
}

ERROR_CHECKING!()