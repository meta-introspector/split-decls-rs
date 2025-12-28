macro_rules! deps {
    () => {
        SyntaxNode!();
    };
}

macro_rules! AstNode {
    () => {
        deps!();
        # [doc = " The main trait to go from untyped `SyntaxNode`  to a typed ast. The"] # [doc = " conversion itself has zero runtime cost: ast and syntax nodes have exactly"] # [doc = " the same representation: a pointer to the tree root and a pointer to the"] # [doc = " node itself."] pub trait AstNode { # [doc = " This panics if the `SyntaxKind` is not statically known."] fn kind () -> SyntaxKind where Self : Sized , { panic ! ("dynamic `SyntaxKind` for `AstNode::kind()`") } fn can_cast (kind : SyntaxKind) -> bool where Self : Sized ; fn cast (syntax : SyntaxNode) -> Option < Self > where Self : Sized ; fn syntax (& self) -> & SyntaxNode ; fn clone_for_update (& self) -> Self where Self : Sized , { Self :: cast (self . syntax () . clone_for_update ()) . unwrap () } fn clone_subtree (& self) -> Self where Self : Sized , { Self :: cast (self . syntax () . clone_subtree ()) . unwrap () } }
    };
}

AstNode!();