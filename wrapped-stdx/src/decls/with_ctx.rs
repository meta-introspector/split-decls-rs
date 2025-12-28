macro_rules! with_ctx {
    () => {
        fn with_ctx (f : impl FnOnce (& mut Vec < String >)) { thread_local ! { static CTX : RefCell < Vec < String >> = const { RefCell :: new (Vec :: new ()) } ; } CTX . with (| ctx | f (& mut ctx . borrow_mut ())) ; }
    };
}

with_ctx!();