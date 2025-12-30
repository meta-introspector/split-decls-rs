// Generated macro for my_attr (function)
macro_rules! Depcratemy_attr {
() => {
// Module: crate
// Provides: {"my_attr"}
// Dependencies: {}
# [proc_macro_attribute] pub fn my_attr (attr : TokenStream , _item : TokenStream) -> TokenStream { let attr = parse :: < MyAttr > (attr) . unwrap () ; let msg = attr . msg . value () ; quote ! (const MSG : & str = # msg ;) . into () }
};
}
