macro_rules! macro_316 {
    () => {
        cfg_rt ! { mod rt ; pub (crate) use rt :: RngSeedGenerator ; cfg_unstable ! { mod rt_unstable ; } }
    };
}

macro_316!()