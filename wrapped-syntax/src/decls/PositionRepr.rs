macro_rules! deps {
    () => {
        SyntaxElement!();
        SyntaxNode!();
    };
}

macro_rules! PositionRepr {
    () => {
        deps!();
        # [derive (Debug)] enum PositionRepr { FirstChild (SyntaxNode) , After (SyntaxElement) , }
    };
}

PositionRepr!();