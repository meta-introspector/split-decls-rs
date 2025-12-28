macro_rules! macro_3 {
    () => {
        cfg_net ! { # [cfg (not (target_arch = "wasm32"))] pub mod udp ; pub mod net ; }
    };
}

macro_3!()