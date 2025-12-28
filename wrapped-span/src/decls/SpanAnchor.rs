macro_rules! deps {
    () => {
        EditionedFileId!();
        ErasedFileAstId!();
    };
}

macro_rules! SpanAnchor {
    () => {
        deps!();
        # [derive (Copy , Clone , PartialEq , Eq , Hash)] pub struct SpanAnchor { pub file_id : EditionedFileId , pub ast_id : ErasedFileAstId , }
    };
}

SpanAnchor!();