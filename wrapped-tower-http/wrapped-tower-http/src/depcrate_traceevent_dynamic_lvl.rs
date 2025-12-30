// Generated macro for event_dynamic_lvl (macro)
macro_rules! Depcrate_traceevent_dynamic_lvl {
() => {
// Module: crate::trace
// Provides: {"event_dynamic_lvl"}
// Dependencies: {}
macro_rules ! event_dynamic_lvl { ($ (target : $ target : expr ,) ? $ (parent : $ parent : expr ,) ? $ lvl : expr , $ ($ tt : tt) *) => { match $ lvl { tracing :: Level :: ERROR => { tracing :: event ! ($ (target : $ target ,) ? $ (parent : $ parent ,) ? tracing :: Level :: ERROR , $ ($ tt) *) ; } tracing :: Level :: WARN => { tracing :: event ! ($ (target : $ target ,) ? $ (parent : $ parent ,) ? tracing :: Level :: WARN , $ ($ tt) *) ; } tracing :: Level :: INFO => { tracing :: event ! ($ (target : $ target ,) ? $ (parent : $ parent ,) ? tracing :: Level :: INFO , $ ($ tt) *) ; } tracing :: Level :: DEBUG => { tracing :: event ! ($ (target : $ target ,) ? $ (parent : $ parent ,) ? tracing :: Level :: DEBUG , $ ($ tt) *) ; } tracing :: Level :: TRACE => { tracing :: event ! ($ (target : $ target ,) ? $ (parent : $ parent ,) ? tracing :: Level :: TRACE , $ ($ tt) *) ; } } } ; }
};
}
