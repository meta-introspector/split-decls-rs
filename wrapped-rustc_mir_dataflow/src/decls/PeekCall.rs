macro_rules! deps {
    () => {
        PeekCallKind!();
    };
}

macro_rules! PeekCall {
    () => {
        deps!();
        # [derive (Clone , Copy , Debug)] struct PeekCall { arg : Local , kind : PeekCallKind , span : Span , }
    };
}

PeekCall!()