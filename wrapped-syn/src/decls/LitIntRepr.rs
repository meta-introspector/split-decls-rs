macro_rules! LitIntRepr {
    () => {
        struct LitIntRepr { token : Literal , digits : Box < str > , suffix : Box < str > , }
    };
}

LitIntRepr!();