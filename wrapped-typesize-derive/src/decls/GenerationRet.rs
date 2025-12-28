macro_rules! GenerationRet {
    () => {
        struct GenerationRet { extra_size : TokenStream , # [cfg (feature = "details")] details : Option < TokenStream > , }
    };
}

GenerationRet!();