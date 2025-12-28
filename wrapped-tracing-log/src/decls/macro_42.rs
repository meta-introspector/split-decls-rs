macro_rules! macro_42 {
    () => {
        log_cs ! (tracing_core :: Level :: INFO , INFO_CS , INFO_META , InfoCallsite) ;
    };
}

macro_42!();