macro_rules! deps {
    () => {
        ErasedHasNameFileAstId!();
        ErasedFileAstId!();
    };
}

macro_rules! ErasedAssocItemFileAstId {
    () => {
        deps!();
        # [doc = " This holds the ast ID for variants too (they're a kind of assoc item)."] # [derive (Hash)] struct ErasedAssocItemFileAstId < 'a > { # [doc = " Subtle: items in `extern` blocks **do not** store the ID of the extern block here."] # [doc = " Instead this is left empty. The reason is that `ExternBlockFileAstId` is pretty unstable"] # [doc = " (it contains only an index), and extern blocks don't introduce a new scope, so storing"] # [doc = " the extern block ID will do more harm to incrementality than help."] parent : Option < ErasedFileAstId > , properties : ErasedHasNameFileAstId < 'a > , }
    };
}

ErasedAssocItemFileAstId!()