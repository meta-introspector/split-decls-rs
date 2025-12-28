macro_rules! deps {
    () => {
        SyntaxNode!();
        SyntaxElement!();
    };
}

macro_rules! PositionRepr {
    () => {
        deps!();
        # [derive (Debug)] enum PositionRepr { FirstChild (SyntaxNode) , After (SyntaxElement) , }
    };
}

PositionRepr!()