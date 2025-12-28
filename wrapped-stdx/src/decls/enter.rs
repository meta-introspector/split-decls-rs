macro_rules! deps {
    () => {
        PanicContext!();
    };
}

macro_rules! enter {
    () => {
        deps!();
        pub fn enter (frame : String) -> PanicContext { # [expect (clippy :: print_stderr , reason = "already panicking anyway")] fn set_hook () { let default_hook = panic :: take_hook () ; panic :: set_hook (Box :: new (move | panic_info | { with_ctx (| ctx | { if ! ctx . is_empty () { eprintln ! ("Panic context:") ; for frame in ctx . iter () { eprintln ! ("> {frame}\n") ; } } }) ; default_hook (panic_info) ; })) ; } static SET_HOOK : Once = Once :: new () ; SET_HOOK . call_once (set_hook) ; with_ctx (| ctx | ctx . push (frame)) ; PanicContext { _priv : () } }
    };
}

enter!()