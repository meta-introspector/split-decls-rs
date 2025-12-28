macro_rules! deps {
    () => {
        Span!();
        ErasedFileAstIdKind!();
        ErasedFileAstId!();
    };
}

macro_rules! FIXUP_ERASED_FILE_AST_ID_MARKER {
    () => {
        deps!();
        # [doc = " ErasedFileAstId used as the span for syntax node fixups. Any Span containing this file id is to be"] # [doc = " considered fake."] pub const FIXUP_ERASED_FILE_AST_ID_MARKER : ErasedFileAstId = ErasedFileAstId (pack_hash_index_and_kind (0 , 0 , ErasedFileAstIdKind :: Fixup as u32)) ;
    };
}

FIXUP_ERASED_FILE_AST_ID_MARKER!();