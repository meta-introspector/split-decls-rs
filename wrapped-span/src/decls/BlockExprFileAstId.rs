macro_rules! deps {
    () => {
        ErasedFileAstId!();
    };
}

macro_rules! BlockExprFileAstId {
    () => {
        deps!();
        # [derive (Debug , Clone , PartialEq , Eq , Hash)] struct BlockExprFileAstId { parent : Option < ErasedFileAstId > , }
    };
}

BlockExprFileAstId!();