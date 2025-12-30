// Generated macro for seq_parse_struct_fields (macro)
macro_rules! Depcrate_macros_seqseq_parse_struct_fields {
() => {
// Module: crate::macros::seq
// Provides: {"seq_parse_struct_fields"}
// Dependencies: {}
# [macro_export] # [doc (hidden)] macro_rules ! seq_parse_struct_fields { ((_ : $ head_parser : expr , $ ($ fields : tt) *) ; ($ unnamed1 : ident , $ ($ unnamed : ident) ,*) ; $ input : ident ;) => { let $ unnamed1 = $ crate :: Parser :: parse_next (& mut $ head_parser , $ input) ?; $ crate :: seq_parse_struct_fields ! (($ ($ fields) *) ; ($ ($ unnamed) ,*) ; $ input ;) } ; ((_ : $ head_parser : expr) ; ($ unnamed1 : ident , $ ($ unnamed : ident) ,*) ; $ input : ident ;) => { let $ unnamed1 = $ crate :: Parser :: parse_next (& mut $ head_parser , $ input) ?; } ; (($ head_field : ident : $ head_parser : expr , $ ($ fields : tt) *) ; ($ unnamed1 : ident , $ ($ unnamed : ident) ,*) ; $ input : ident ;) => { let $ head_field = $ crate :: Parser :: parse_next (& mut $ head_parser , $ input) ?; $ crate :: seq_parse_struct_fields ! (($ ($ fields) *) ; ($ ($ unnamed) ,*) ; $ input ;) } ; (($ head_field : ident : $ head_parser : expr) ; ($ unnamed1 : ident , $ ($ unnamed : ident) ,*) ; $ input : ident ;) => { let $ head_field = $ crate :: Parser :: parse_next (& mut $ head_parser , $ input) ?; } ; ((.. $ update : expr) ; ($ ($ unnamed : ident) ,*) ; $ input : expr ;) => { } ; (($ (,) ?) ; ($ ($ unnamed : ident) ,*) ; $ input : expr ;) => { } ; }
};
}
