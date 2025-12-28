macro_rules! ErasedHasNameFileAstId {
    () => {
        # [derive (Hash)] struct ErasedHasNameFileAstId < 'a > { name : & 'a str , }
    };
}

ErasedHasNameFileAstId!()