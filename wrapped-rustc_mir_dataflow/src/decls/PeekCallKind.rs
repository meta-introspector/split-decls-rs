macro_rules! PeekCallKind {
    () => {
        # [derive (Clone , Copy , Debug)] enum PeekCallKind { ByVal , ByRef , }
    };
}

PeekCallKind!()