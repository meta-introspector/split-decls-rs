// Generated macro for MaybeItemFnRef (struct)
macro_rules! DepcrateMaybeItemFnRef {
() => {
// Module: crate
// Provides: {"MaybeItemFnRef"}
// Dependencies: {}
# [doc = " A generic reference type for `MaybeItemFn`,"] # [doc = " that takes a generic block type `B` that implements `ToTokens` (eg. `TokenStream`, `Block`)."] # [derive (Debug , Clone)] struct MaybeItemFnRef < 'a , B : ToTokens > { outer_attrs : & 'a Vec < Attribute > , inner_attrs : & 'a Vec < Attribute > , vis : & 'a Visibility , sig : & 'a Signature , brace_token : & 'a Brace , block : & 'a B , }
};
}
