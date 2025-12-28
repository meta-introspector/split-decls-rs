macro_rules! deps {
    () => {
        InterestCacheConfig!();
    };
}

macro_rules! CONFIG {
    () => {
        deps!();
        static CONFIG : Lazy < Mutex < InterestCacheConfig > > = Lazy :: new (| | { tracing_core :: callsite :: register (& SENTINEL_CALLSITE) ; Mutex :: new (InterestCacheConfig :: disabled ()) }) ;
    };
}

CONFIG!();