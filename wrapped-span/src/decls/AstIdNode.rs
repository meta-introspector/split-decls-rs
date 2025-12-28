macro_rules! AstIdNode {
    () => {
        pub trait AstIdNode : AstNode { }
    };
}

AstIdNode!();