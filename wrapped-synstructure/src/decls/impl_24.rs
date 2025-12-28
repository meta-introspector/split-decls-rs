macro_rules! deps {
    () => {
        MacroResult!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        # [cfg (all (not (all (target_arch = "wasm32" , any (target_os = "unknown" , target_os = "wasi"))) , feature = "proc-macro"))] impl MacroResult for proc_macro :: TokenStream { fn into_result (self) -> Result < TokenStream > { Ok (self . into ()) } fn into_stream (self) -> proc_macro :: TokenStream { self } }
    };
}

impl_24!()