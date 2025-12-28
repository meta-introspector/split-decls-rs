macro_rules! macro_1 {
    () => {
        # [cfg (all (not (tokio_unstable) , target_family = "wasm" , any (feature = "fs" , feature = "io-std" , feature = "net" , feature = "process" , feature = "rt-multi-thread" , feature = "signal")))] compile_error ! ("Only features sync,macros,io-util,rt,time are supported on wasm.") ;
    };
}

macro_1!()