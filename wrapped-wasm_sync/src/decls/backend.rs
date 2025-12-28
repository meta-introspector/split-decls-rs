macro_rules! backend {
    () => {
        # [doc = " Provides the ability to generate and compare type IDs in a `const` context."] # [cfg_attr (all (target_arch = "wasm32" , target_feature = "atomics") , path = "wasm.rs")] # [cfg_attr (not (all (target_arch = "wasm32" , target_feature = "atomics")) , path = "native.rs")] mod backend ;
    };
}

backend!()