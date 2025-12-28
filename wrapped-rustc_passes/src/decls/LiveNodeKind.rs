macro_rules! LiveNodeKind {
    () => {
        # [derive (Copy , Clone , PartialEq , Debug)] enum LiveNodeKind { UpvarNode (Span) , ExprNode (Span , HirId) , VarDefNode (Span , HirId) , ClosureNode , ExitNode , }
    };
}

LiveNodeKind!()