macro_rules! deps {
    () => {
        ErasedFileAstId!();
    };
}

macro_rules! ArenaId {
    () => {
        deps!();
        type ArenaId = Idx < (SyntaxNodePtr , ErasedFileAstId) > ;
    };
}

ArenaId!();