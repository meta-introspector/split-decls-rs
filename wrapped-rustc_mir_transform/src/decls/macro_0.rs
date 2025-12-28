macro_rules! macro_0 {
    () => {
        thread_local ! { # [doc = " Maps MIR pass names to a snake case form to match profiling naming style"] static PASS_TO_PROFILER_NAMES : RefCell < FxHashMap <&'static str , &'static str >> = { RefCell :: new (FxHashMap :: default ()) } ; }
    };
}

macro_0!();