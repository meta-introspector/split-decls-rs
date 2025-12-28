macro_rules! macro_90 {
    () => {
        cfg_trace ! { mod trace ; # [allow (unused_imports)] pub (crate) use trace :: InstrumentedFuture as Future ; }
    };
}

macro_90!();