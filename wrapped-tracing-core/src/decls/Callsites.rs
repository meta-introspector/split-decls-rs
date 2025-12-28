macro_rules! deps {
    () => {
        DefaultCallsite!();
    };
}

macro_rules! Callsites {
    () => {
        deps!();
        struct Callsites { list_head : AtomicPtr < DefaultCallsite > , has_locked_callsites : AtomicBool , }
    };
}

Callsites!();