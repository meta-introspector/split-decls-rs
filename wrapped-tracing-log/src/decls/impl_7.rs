macro_rules! deps {
    () => {
        Builder!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl Default for Builder { fn default () -> Self { Self { ignore_crates : Vec :: new () , filter : log :: LevelFilter :: max () , # [cfg (all (feature = "interest-cache" , feature = "std"))] interest_cache_config : None , } } }
    };
}

impl_7!()