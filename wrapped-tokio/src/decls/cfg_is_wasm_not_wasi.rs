macro_rules! cfg_is_wasm_not_wasi {
    () => {
        macro_rules ! cfg_is_wasm_not_wasi { ($ ($ item : item) *) => { $ (# [cfg (all (target_family = "wasm" , not (target_os = "wasi")))] $ item) * } }
    };
}

cfg_is_wasm_not_wasi!()