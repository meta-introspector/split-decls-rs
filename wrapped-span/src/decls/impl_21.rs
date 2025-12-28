macro_rules! deps {
    () => {
        FileAstId!();
        AstIdNode!();
        ErasedFileAstId!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl < N > FileAstId < N > { # [inline] pub fn upcast < M : AstIdNode > (self) -> FileAstId < M > where N : Into < M > , { FileAstId { raw : self . raw , _marker : PhantomData } } # [inline] pub fn erase (self) -> ErasedFileAstId { self . raw } }
    };
}

impl_21!();