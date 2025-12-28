macro_rules! deps {
    () => {
        ErasedFileAstIdKind!();
    };
}

macro_rules! ErasedAstIdNextIndexMap {
    () => {
        deps!();
        # [derive (Default)] struct ErasedAstIdNextIndexMap (FxHashMap < (ErasedFileAstIdKind , u16) , u32 >) ;
    };
}

ErasedAstIdNextIndexMap!()