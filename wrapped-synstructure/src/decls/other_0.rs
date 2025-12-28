macro_rules! other_0 {
    () => {
        # [cfg (all (not (all (target_arch = "wasm32" , any (target_os = "unknown" , target_os = "wasi"))) , feature = "proc-macro"))] extern crate proc_macro ;
    };
}

other_0!();