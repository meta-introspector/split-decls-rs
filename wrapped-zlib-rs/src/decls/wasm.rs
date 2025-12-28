macro_rules! wasm {
    () => {
        # [cfg (any (target_arch = "wasm32" , target_arch = "wasm64"))] mod wasm ;
    };
}

wasm!();