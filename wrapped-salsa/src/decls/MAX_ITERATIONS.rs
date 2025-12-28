macro_rules! deps {
    () => {
        IterationCount!();
    };
}

macro_rules! MAX_ITERATIONS {
    () => {
        deps!();
        # [doc = " The maximum number of times we'll fixpoint-iterate before panicking."] # [doc = ""] # [doc = " Should only be relevant in case of a badly configured cycle recovery."] pub const MAX_ITERATIONS : IterationCount = IterationCount (200) ;
    };
}

MAX_ITERATIONS!()