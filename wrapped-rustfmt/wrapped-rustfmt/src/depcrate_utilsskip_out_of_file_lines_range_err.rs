// Generated macro for skip_out_of_file_lines_range_err (macro)
macro_rules! Depcrate_utilsskip_out_of_file_lines_range_err {
() => {
// Module: crate::utils
// Provides: {"skip_out_of_file_lines_range_err"}
// Dependencies: {}
macro_rules ! skip_out_of_file_lines_range_err { ($ self : ident , $ span : expr) => { if out_of_file_lines_range ! ($ self , $ span) { return Err (RewriteError :: SkipFormatting) ; } } ; }
};
}
