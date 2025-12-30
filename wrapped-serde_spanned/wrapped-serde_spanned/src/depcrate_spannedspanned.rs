// Generated macro for Spanned (struct)
macro_rules! Depcrate_spannedSpanned {
() => {
// Module: crate::spanned
// Provides: {"Spanned"}
// Dependencies: {}
# [doc = " A spanned value, indicating the range at which it is defined in the source."] # [derive (Clone , Debug)] pub struct Spanned < T > { # [doc = " Byte range"] span : core :: ops :: Range < usize > , # [doc = " The spanned value."] value : T , }
};
}
