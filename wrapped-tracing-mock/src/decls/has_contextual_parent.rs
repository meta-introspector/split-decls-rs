macro_rules! deps {
    () => {
        ExpectedSpan!();
        ExpectedAncestry!();
    };
}

macro_rules! has_contextual_parent {
    () => {
        deps!();
        # [doc = " Convenience function that returns [`ExpectedAncestry::HasContextualParent`] with"] # [doc = " provided name."] pub fn has_contextual_parent < S : Into < ExpectedSpan > > (span : S) -> ExpectedAncestry { ExpectedAncestry :: HasContextualParent (span . into ()) }
    };
}

has_contextual_parent!()