// Generated macro for MaybeItemFn (struct)
macro_rules! DepcrateMaybeItemFn {
() => {
// Module: crate
// Provides: {"MaybeItemFn"}
// Dependencies: {}
# [doc = " This is a more flexible/imprecise `ItemFn` type,"] # [doc = " which's block is just a `TokenStream` (it may contain invalid code)."] # [derive (Debug , Clone)] struct MaybeItemFn { outer_attrs : Vec < Attribute > , inner_attrs : Vec < Attribute > , vis : Visibility , sig : Signature , brace_token : Brace , block : TokenStream , }
};
}
