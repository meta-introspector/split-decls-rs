// Generated macro for seq_init_tuple_fields (macro)
macro_rules! Depcrate_macros_seqseq_init_tuple_fields {
() => {
// Module: crate::macros::seq
// Provides: {"seq_init_tuple_fields"}
// Dependencies: {}
# [macro_export] # [doc (hidden)] macro_rules ! seq_init_tuple_fields { ((_ : $ head_parser : expr , $ ($ fields : tt) *) ; ($ unnamed1 : ident , $ ($ unnamed : ident) ,*) ; $ ($ name : ident) ::*; $ ($ inits : tt) *) => { $ crate :: seq_init_tuple_fields ! (($ ($ fields) *) ; ($ ($ unnamed) ,*) ; $ ($ name) ::* ; $ ($ inits) *) } ; ((_ : $ head_parser : expr) ; ($ unnamed1 : ident , $ ($ unnamed : ident) ,*) ; $ ($ name : ident) ::*; $ ($ inits : tt) *) => { $ crate :: seq_init_tuple_fields ! (() ; ($ ($ unnamed) ,*) ; $ ($ name) ::* ; $ ($ inits) *) } ; (($ head_parser : expr , $ ($ fields : tt) *) ; ($ unnamed1 : ident , $ ($ unnamed : ident) ,*) ; $ ($ name : ident) ::*; $ ($ inits : tt) *) => { $ crate :: seq_init_tuple_fields ! (($ ($ fields) *) ; ($ ($ unnamed) ,*) ; $ ($ name) ::* ; $ ($ inits) * $ unnamed1 ,) } ; (($ head_parser : expr) ; ($ unnamed1 : ident , $ ($ unnamed : ident) ,*) ; $ ($ name : ident) ::*; $ ($ inits : tt) *) => { $ crate :: seq_init_tuple_fields ! (() ; ($ ($ unnamed) ,*) ; $ ($ name) ::* ; $ ($ inits) * $ unnamed1 ,) } ; (($ (,) ?) ; ($ unnamed1 : ident , $ ($ unnamed : ident) ,*) ; $ ($ name : ident) ::*; $ ($ inits : tt) *) => { $ ($ name) ::* ($ ($ inits) *) } ; }
};
}
