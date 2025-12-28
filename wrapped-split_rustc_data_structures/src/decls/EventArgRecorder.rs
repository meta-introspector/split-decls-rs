macro_rules! deps {
    () => {
        SelfProfiler!();
    };
}

macro_rules! EventArgRecorder {
    () => {
        deps!();
        # [doc = " A helper for recording costly arguments to self-profiling events. Used with"] # [doc = " `SelfProfilerRef::generic_activity_with_arg_recorder`."] pub struct EventArgRecorder < 'p > { # [doc = " The `SelfProfiler` used to intern the event arguments that users will ask to record."] profiler : & 'p SelfProfiler , # [doc = " The interned event arguments to be recorded in the generic activity event."] # [doc = ""] # [doc = " The most common case, when actually recording event arguments, is to have one argument. Then"] # [doc = " followed by recording two, in a couple places."] args : SmallVec < [StringId ; 2] > , }
    };
}

EventArgRecorder!()