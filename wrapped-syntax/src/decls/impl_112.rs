macro_rules! deps {
    () => {
        SyntaxNode!();
        AstNode!();
    };
}

macro_rules! impl_112 {
    () => {
        deps!();
        impl < L , R > AstNode for Either < L , R > where L : AstNode , R : AstNode , { fn can_cast (kind : SyntaxKind) -> bool where Self : Sized , { L :: can_cast (kind) || R :: can_cast (kind) } fn cast (syntax : SyntaxNode) -> Option < Self > where Self : Sized , { if L :: can_cast (syntax . kind ()) { L :: cast (syntax) . map (Either :: Left) } else { R :: cast (syntax) . map (Either :: Right) } } fn syntax (& self) -> & SyntaxNode { self . as_ref () . either (L :: syntax , R :: syntax) } }
    };
}

impl_112!();