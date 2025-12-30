// Generated macro for thread (module)
macro_rules! Depcrate_loom_stdthread {
() => {
// Module: crate::loom::std
// Provides: {"thread"}
// Dependencies: {}
pub (crate) mod thread { # [inline] pub (crate) fn yield_now () { std :: hint :: spin_loop () ; } # [allow (unused_imports)] pub (crate) use std :: thread :: { current , panicking , park , park_timeout , sleep , spawn , AccessError , Builder , JoinHandle , LocalKey , Result , Thread , ThreadId , } ; }
};
}
