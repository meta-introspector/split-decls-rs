macro_rules! SPAN_TRACK {
    () => {
        pub static SPAN_TRACK : AtomicRef < fn (LocalDefId) > = AtomicRef :: new (& ((| _ | { }) as fn (_))) ;
    };
}

SPAN_TRACK!()