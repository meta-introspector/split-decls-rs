macro_rules! AliasBoundKind {
    () => {
        enum AliasBoundKind { SelfBounds , NonSelfBounds , }
    };
}

AliasBoundKind!()