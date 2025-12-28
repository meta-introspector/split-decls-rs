macro_rules! deps {
    () => {
        InterestCacheConfig!();
        LogTracer!();
    };
}

macro_rules! Builder {
    () => {
        deps!();
        # [doc = " Configures a new `LogTracer`."] # [derive (Debug)] pub struct Builder { ignore_crates : Vec < String > , filter : log :: LevelFilter , # [cfg (all (feature = "interest-cache" , feature = "std"))] interest_cache_config : Option < crate :: InterestCacheConfig > , }
    };
}

Builder!();