macro_rules! deps {
    () => {
        SyntaxEditor!();
        SyntaxNode!();
        SyntaxElement!();
        SyntaxAnnotation!();
    };
}

macro_rules! SyntaxEdit {
    () => {
        deps!();
        # [doc = " Represents a completed [`SyntaxEditor`] operation."] pub struct SyntaxEdit { old_root : SyntaxNode , new_root : SyntaxNode , changed_elements : Vec < SyntaxElement > , annotations : FxHashMap < SyntaxAnnotation , Vec < SyntaxElement > > , }
    };
}

SyntaxEdit!()