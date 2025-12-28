macro_rules! deps {
    () => {
        MaybeItemFn!();
    };
}

macro_rules! MaybeItemFnRef {
    () => {
        deps!();
        # [doc = " A generic reference type for `MaybeItemFn`,"] # [doc = " that takes a generic block type `B` that implements `ToTokens` (eg. `TokenStream`, `Block`)."] # [derive (Debug , Clone)] struct MaybeItemFnRef < 'a , B : ToTokens > { outer_attrs : & 'a Vec < Attribute > , inner_attrs : & 'a Vec < Attribute > , vis : & 'a Visibility , sig : & 'a Signature , brace_token : & 'a Brace , block : & 'a B , }
    };
}

MaybeItemFnRef!()