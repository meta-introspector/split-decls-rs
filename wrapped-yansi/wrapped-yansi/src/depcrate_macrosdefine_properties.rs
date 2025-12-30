// Generated macro for define_properties (macro)
macro_rules! Depcrate_macrosdefine_properties {
() => {
// Module: crate::macros
// Provides: {"define_properties"}
// Dependencies: {}
macro_rules ! define_properties { ($ ($ (# [$ attr : meta]) * $ kind : ident ($ A : ident) $ ({ $ ($ t : tt) * }) ?) ,* $ (,) ?) => { $ (define_property ! ($ (# [$ attr]) * $ kind ($ A) $ ({ $ ($ t) * }) ?) ;) * $ (check_property_exhaustiveness ! ($ A $ ({ $ ($ t) * }) ?) ;) * } }
};
}
