macro_rules! SourceRootKind {
    () => {
        # [derive (Debug , Clone , Copy)] enum SourceRootKind { Local , Library , }
    };
}

SourceRootKind!()