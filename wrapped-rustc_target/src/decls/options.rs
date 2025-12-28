macro_rules! deps {
    () => {
        LinkSelfContainedDefault!();
        LinkerFlavor!();
        Cc!();
        TargetOptions!();
    };
}

macro_rules! options {
    () => {
        deps!();
        pub (crate) fn options () -> TargetOptions { macro_rules ! args { ($ prefix : literal) => { & [concat ! ($ prefix , "-z") , concat ! ($ prefix , "stack-size=1048576") , concat ! ($ prefix , "--stack-first") , concat ! ($ prefix , "--allow-undefined") , concat ! ($ prefix , "--no-demangle") ,] } ; } let mut pre_link_args = TargetOptions :: link_args (LinkerFlavor :: WasmLld (Cc :: No) , args ! ("")) ; add_link_args (& mut pre_link_args , LinkerFlavor :: WasmLld (Cc :: Yes) , args ! ("-Wl,")) ; TargetOptions { is_like_wasm : true , binary_format : BinaryFormat :: Wasm , families : cvs ! ["wasm"] , dynamic_linking : true , only_cdylib : true , exe_suffix : ".wasm" . into () , dll_prefix : "" . into () , dll_suffix : ".wasm" . into () , eh_frame_header : false , max_atomic_width : Some (64) , panic_strategy : PanicStrategy :: Abort , singlethread : true , limit_rdylib_exports : false , linker : Some ("rust-lld" . into ()) , linker_flavor : LinkerFlavor :: WasmLld (Cc :: No) , pre_link_args , link_self_contained : LinkSelfContainedDefault :: True , relocation_model : RelocModel :: Static , has_thread_local : true , tls_model : TlsModel :: LocalExec , emit_debug_gdb_scripts : false , generate_arange_section : false , .. Default :: default () } }
    };
}

options!()