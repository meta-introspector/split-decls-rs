// Generated macro for seq_init_struct_fields (macro)
macro_rules! Depcrate_macros_seqseq_init_struct_fields {
() => {
// Module: crate::macros::seq
// Provides: {"seq_init_struct_fields"}
// Dependencies: {}
# [macro_export] # [doc (hidden)] macro_rules ! seq_init_struct_fields { ((_ : $ head_parser : expr , $ ($ fields : tt) *) ; $ ($ name : ident) ::* ; $ ($ inits : tt) *) => { $ crate :: seq_init_struct_fields ! (($ ($ fields) *) ; $ ($ name) ::* ; $ ($ inits) *) } ; ((_ : $ head_parser : expr) ; $ ($ name : ident) ::* ; $ ($ inits : tt) *) => { $ crate :: seq_init_struct_fields ! (() ; $ ($ name) ::* ; $ ($ inits) *) } ; (($ head_field : ident : $ head_parser : expr , $ ($ fields : tt) *) ; $ ($ name : ident) ::* ; $ ($ inits : tt) *) => { $ crate :: seq_init_struct_fields ! (($ ($ fields) *) ; $ ($ name) ::* ; $ ($ inits) * $ head_field ,) } ; (($ head_field : ident : $ head_parser : expr) ; $ ($ name : ident) ::* ; $ ($ inits : tt) *) => { $ crate :: seq_init_struct_fields ! (() ; $ ($ name) ::* ; $ ($ inits) * $ head_field ,) } ; ((.. $ update : expr) ; $ ($ name : ident) ::* ; $ ($ inits : tt) *) => { $ ($ name) ::* { $ ($ inits) * ..$ update } } ; (($ (,) ?) ; $ ($ name : ident) ::* ; $ ($ inits : tt) *) => { $ ($ name) ::* { $ ($ inits) * } } ; }
};
}
