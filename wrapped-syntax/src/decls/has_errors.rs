macro_rules! deps {
    () => {
        SyntaxNode!();
    };
}

macro_rules! has_errors {
    () => {
        deps!();
        pub fn has_errors (node : & SyntaxNode) -> bool { node . children () . any (| it | it . kind () == SyntaxKind :: ERROR) }
    };
}

has_errors!()