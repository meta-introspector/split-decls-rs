macro_rules! MacroResult {
    () => {
        # [doc = " Helper trait describing values which may be returned by macro implementation"] # [doc = " methods used by this crate's macros."] pub trait MacroResult { # [doc = " Convert this result into a `Result` for further processing / validation."] fn into_result (self) -> Result < TokenStream > ; # [doc = " Convert this result into a `proc_macro::TokenStream`, ready to return"] # [doc = " from a native `proc_macro` implementation."] # [doc = ""] # [doc = " If `into_result()` would return an `Err`, this method should instead"] # [doc = " generate a `compile_error!` invocation to nicely report the error."] # [doc = ""] # [doc = " *This method is available if `synstructure` is built with the"] # [doc = " `\"proc-macro\"` feature.*"] # [cfg (all (not (all (target_arch = "wasm32" , any (target_os = "unknown" , target_os = "wasi"))) , feature = "proc-macro"))] fn into_stream (self) -> proc_macro :: TokenStream where Self : Sized , { match self . into_result () { Ok (ts) => ts . into () , Err (err) => err . to_compile_error () . into () , } } }
    };
}

MacroResult!()