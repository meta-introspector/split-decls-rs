macro_rules! deps {
    () => {
        LocalDefId!();
    };
}

macro_rules! SPAN_TRACK {
    () => {
        deps!();
        pub static SPAN_TRACK : AtomicRef < fn (LocalDefId) > = AtomicRef :: new (& ((| _ | { }) as fn (_))) ;
    };
}

SPAN_TRACK!()