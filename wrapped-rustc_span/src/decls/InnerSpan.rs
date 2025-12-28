macro_rules! InnerSpan {
    () => {
        # [doc = " Range inside of a `Span` used for diagnostics when we only have access to relative positions."] # [derive (Copy , Clone , PartialEq , Eq , Debug)] pub struct InnerSpan { pub start : usize , pub end : usize , }
    };
}

InnerSpan!()