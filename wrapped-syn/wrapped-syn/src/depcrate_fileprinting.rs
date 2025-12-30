// Generated macro for printing (module)
macro_rules! Depcrate_fileprinting {
() => {
// Module: crate::file
// Provides: {"printing"}
// Dependencies: {}
# [cfg (feature = "printing")] mod printing { use super :: * ; use attr :: FilterAttrs ; use quote :: { Tokens , ToTokens } ; impl ToTokens for File { fn to_tokens (& self , tokens : & mut Tokens) { tokens . append_all (self . attrs . inner ()) ; tokens . append_all (& self . items) ; } } }
};
}
