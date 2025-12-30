// Generated macro for callsite2 (macro)
macro_rules! Depcrate_macroscallsite2 {
() => {
// Module: crate::macros
// Provides: {"callsite2"}
// Dependencies: {}
# [doc = " Constructs a new static callsite for a span or event."] # [doc (hidden)] # [macro_export] macro_rules ! callsite2 { (name : $ name : expr , kind : $ kind : expr , fields : $ ($ fields : tt) *) => { { $ crate :: callsite2 ! { name : $ name , kind : $ kind , target : module_path ! () , level : $ crate :: Level :: TRACE , fields : $ ($ fields) * } } } ; (name : $ name : expr , kind : $ kind : expr , level : $ lvl : expr , fields : $ ($ fields : tt) *) => { { $ crate :: callsite2 ! { name : $ name , kind : $ kind , target : module_path ! () , level : $ lvl , fields : $ ($ fields) * } } } ; (name : $ name : expr , kind : $ kind : expr , target : $ target : expr , level : $ lvl : expr , fields : $ ($ fields : tt) *) => { { static META : $ crate :: Metadata <'static > = { $ crate :: metadata ! { name : $ name , target : $ target , level : $ lvl , fields : $ crate :: fieldset ! ($ ($ fields) *) , callsite : & __CALLSITE , kind : $ kind , } } ; $ crate :: callsite :: DefaultCallsite :: new (& META) } } ; }
};
}
