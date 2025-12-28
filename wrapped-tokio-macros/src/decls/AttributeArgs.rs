macro_rules! AttributeArgs {
    () => {
        type AttributeArgs = syn :: punctuated :: Punctuated < syn :: Meta , syn :: Token ! [,] > ;
    };
}

AttributeArgs!()