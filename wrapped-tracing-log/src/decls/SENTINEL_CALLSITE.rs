macro_rules! deps {
    () => {
        SentinelCallsite!();
    };
}

macro_rules! SENTINEL_CALLSITE {
    () => {
        deps!();
        static SENTINEL_CALLSITE : SentinelCallsite = SentinelCallsite ;
    };
}

SENTINEL_CALLSITE!();