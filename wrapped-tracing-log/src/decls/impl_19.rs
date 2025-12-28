macro_rules! deps {
    () => {
        SentinelCallsite!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl tracing_core :: Callsite for SentinelCallsite { fn set_interest (& self , _ : tracing_core :: subscriber :: Interest) { INTEREST_CACHE_EPOCH . fetch_add (1 , Ordering :: SeqCst) ; } fn metadata (& self) -> & tracing_core :: Metadata < '_ > { & SENTINEL_METADATA } }
    };
}

impl_19!();