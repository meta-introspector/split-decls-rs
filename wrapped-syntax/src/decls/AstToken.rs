macro_rules! deps {
    () => {
        SyntaxToken!();
        AstNode!();
    };
}

macro_rules! AstToken {
    () => {
        deps!();
        # [doc = " Like `AstNode`, but wraps tokens rather than interior nodes."] pub trait AstToken { fn can_cast (token : SyntaxKind) -> bool where Self : Sized ; fn cast (syntax : SyntaxToken) -> Option < Self > where Self : Sized ; fn syntax (& self) -> & SyntaxToken ; fn text (& self) -> & str { self . syntax () . text () } }
    };
}

AstToken!()