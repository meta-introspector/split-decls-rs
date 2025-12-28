macro_rules! deps {
    () => {
        SyntaxElement!();
        Change!();
        SyntaxAnnotation!();
        SyntaxNode!();
    };
}

macro_rules! SyntaxEditor {
    () => {
        deps!();
        # [derive (Debug)] pub struct SyntaxEditor { root : SyntaxNode , changes : Vec < Change > , mappings : SyntaxMapping , annotations : Vec < (SyntaxElement , SyntaxAnnotation) > , }
    };
}

SyntaxEditor!();