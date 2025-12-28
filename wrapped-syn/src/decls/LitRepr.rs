macro_rules! LitRepr {
    () => {
        struct LitRepr { token : Literal , suffix : Box < str > , }
    };
}

LitRepr!();