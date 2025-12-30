// Generated macro for kassert (macro)
macro_rules! Depcratekassert {
() => {
// Module: crate
// Provides: {"kassert"}
// Dependencies: {}
# [macro_export] macro_rules ! kassert { ($ test : expr) => ({ if !$ test { sprintln ! ("kassertion failed: {}, {}:{}:{}" , stringify ! ($ test) , file ! () , line ! () , column ! ()) ; unsafe { x86test :: outw (0xf4 , 0x01) ; } } }) ; ($ test : expr , $ ($ arg : tt) +) => ({ if !$ test { sprintln ! ("kassertion failed: {}, {}:{}:{}" , format_args ! ($ ($ arg) +) , file ! () , line ! () , column ! ()) ; # [allow (unused_unsafe)] unsafe { x86test :: outw (0xf4 , 0x01) ; } } }) ; }
};
}
