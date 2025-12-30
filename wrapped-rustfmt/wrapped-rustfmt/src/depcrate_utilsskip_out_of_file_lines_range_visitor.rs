// Generated macro for skip_out_of_file_lines_range_visitor (macro)
macro_rules! Depcrate_utilsskip_out_of_file_lines_range_visitor {
() => {
// Module: crate::utils
// Provides: {"skip_out_of_file_lines_range_visitor"}
// Dependencies: {}
macro_rules ! skip_out_of_file_lines_range_visitor { ($ self : ident , $ span : expr) => { if out_of_file_lines_range ! ($ self , $ span) { $ self . push_rewrite ($ span , None) ; return ; } } ; }
};
}
