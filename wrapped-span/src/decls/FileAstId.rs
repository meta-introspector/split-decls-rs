macro_rules! deps {
    () => {
        ErasedFileAstId!();
    };
}

macro_rules! FileAstId {
    () => {
        deps!();
        # [doc = " `AstId` points to an AST node in a specific file."] pub struct FileAstId < N > { raw : ErasedFileAstId , _marker : PhantomData < fn () -> N > , }
    };
}

FileAstId!();