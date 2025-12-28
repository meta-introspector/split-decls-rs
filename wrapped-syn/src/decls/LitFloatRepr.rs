macro_rules! LitFloatRepr {
    () => {
        struct LitFloatRepr { token : Literal , digits : Box < str > , suffix : Box < str > , }
    };
}

LitFloatRepr!()