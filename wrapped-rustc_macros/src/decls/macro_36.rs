macro_rules! macro_36 {
    () => {
        thread_local ! { pub (crate) static CODE_IDENT_COUNT : RefCell < u32 > = RefCell :: new (0) ; }
    };
}

macro_36!();