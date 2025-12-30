// Generated macro for seq_parse_tuple_fields (macro)
macro_rules! Depcrate_macros_seqseq_parse_tuple_fields {
() => {
// Module: crate::macros::seq
// Provides: {"seq_parse_tuple_fields"}
// Dependencies: {}
# [macro_export] # [doc (hidden)] macro_rules ! seq_parse_tuple_fields { (($ (_ :) ? $ head_parser : expr , $ ($ fields : tt) *) ; ($ unnamed1 : ident , $ ($ unnamed : ident) ,*) ; $ input : ident ;) => { let $ unnamed1 = $ crate :: Parser :: parse_next (& mut $ head_parser , $ input) ?; $ crate :: seq_parse_tuple_fields ! (($ ($ fields) *) ; ($ ($ unnamed) ,*) ; $ input ;) } ; (($ (_ :) ? $ head_parser : expr) ; ($ unnamed1 : ident , $ ($ unnamed : ident) ,*) ; $ input : ident ;) => { let $ unnamed1 = $ crate :: Parser :: parse_next (& mut $ head_parser , $ input) ?; } ; (($ (,) ?) ; ($ ($ unnamed : ident) ,*) ; $ input : expr ;) => { } ; }
};
}
