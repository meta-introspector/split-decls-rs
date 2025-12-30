// Generated macro for DebugWithContext (trait)
macro_rules! Depcrate_framework_fmtDebugWithContext {
() => {
// Module: crate::framework::fmt
// Provides: {"DebugWithContext"}
// Dependencies: {}
# [doc = " An extension to `fmt::Debug` for data that can be better printed with some auxiliary data `C`."] pub trait DebugWithContext < C > : Eq + fmt :: Debug { fn fmt_with (& self , _ctxt : & C , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (self , f) } # [doc = " Print the difference between `self` and `old`."] # [doc = ""] # [doc = " This should print nothing if `self == old`."] # [doc = ""] # [doc = " `+` and `-` are typically used to indicate differences. However, these characters are"] # [doc = " fairly common and may be needed to print a types representation. If using them to indicate"] # [doc = " a diff, prefix them with the \"Unit Separator\"  control character (␟  U+001F)."] fn fmt_diff_with (& self , old : & Self , ctxt : & C , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if self == old { return Ok (()) ; } write ! (f , "\u{001f}+") ? ; self . fmt_with (ctxt , f) ? ; if f . alternate () { write ! (f , "\n") ? ; } else { write ! (f , "\t") ? ; } write ! (f , "\u{001f}-") ? ; old . fmt_with (ctxt , f) } }
};
}
