macro_rules! deps {
    () => {
        DebugValue!();
        Value!();
    };
}

macro_rules! debug {
    () => {
        deps!();
        # [doc = " Wraps a type implementing `fmt::Debug` as a `Value` that can be"] # [doc = " recorded using its `Debug` implementation."] pub fn debug < T > (t : T) -> DebugValue < T > where T : fmt :: Debug , { DebugValue (t) }
    };
}

debug!()