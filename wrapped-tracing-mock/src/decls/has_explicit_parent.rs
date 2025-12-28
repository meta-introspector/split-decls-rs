macro_rules! deps {
    () => {
        ExpectedSpan!();
        ExpectedAncestry!();
    };
}

macro_rules! has_explicit_parent {
    () => {
        deps!();
        # [doc = " Convenience function that returns [`ExpectedAncestry::HasExplicitParent`] with"] # [doc = " provided name."] pub fn has_explicit_parent < S : Into < ExpectedSpan > > (span : S) -> ExpectedAncestry { ExpectedAncestry :: HasExplicitParent (span . into ()) }
    };
}

has_explicit_parent!();