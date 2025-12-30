// Generated macro for uDisplayHex (trait)
macro_rules! DepcrateuDisplayHex {
() => {
// Module: crate
// Provides: {"uDisplayHex"}
// Dependencies: {}
# [doc = " HEADS UP this is currently an implementation detail and not subject to semver guarantees."] # [doc = " do NOT use this outside the `ufmt` crate"] # [doc (hidden)] # [allow (non_camel_case_types)] pub trait uDisplayHex { # [doc = " Formats the value using the given formatter"] fn fmt_hex < W > (& self , _ : & mut Formatter < '_ , W > , options : HexOptions) -> Result < () , W :: Error > where W : uWrite + ? Sized ; }
};
}
