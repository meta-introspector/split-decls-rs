macro_rules! JodChild {
    () => {
        # [doc = " A [`std::process::Child`] wrapper that will kill the child on drop."] # [cfg_attr (not (target_arch = "wasm32") , repr (transparent))] # [derive (Debug)] pub struct JodChild (pub std :: process :: Child) ;
    };
}

JodChild!();