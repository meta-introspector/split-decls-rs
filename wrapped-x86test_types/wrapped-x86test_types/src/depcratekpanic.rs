// Generated macro for kpanic (macro)
macro_rules! Depcratekpanic {
() => {
// Module: crate
// Provides: {"kpanic"}
// Dependencies: {}
# [macro_export] macro_rules ! kpanic { ($ test : expr) => ({ sprintln ! ("kpanic: {}, {}:{}:{}" , stringify ! ($ test) , file ! () , line ! () , column ! ()) ; unsafe { x86test :: outw (0xf4 , 0x02) ; } }) ; ($ test : expr , $ ($ arg : tt) +) => ({ if !$ test { sprintln ! ("kpanic: {}, {}:{}:{}" , format_args ! ($ ($ arg) +) , file ! () , line ! () , column ! ()) ; # [allow (unused_unsafe)] unsafe { x86test :: outw (0xf4 , 0x02) ; } } }) ; }
};
}
