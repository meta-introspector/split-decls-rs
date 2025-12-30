// Generated macro for out_of_file_lines_range (macro)
macro_rules! Depcrate_utilsout_of_file_lines_range {
() => {
// Module: crate::utils
// Provides: {"out_of_file_lines_range"}
// Dependencies: {}
macro_rules ! out_of_file_lines_range { ($ self : ident , $ span : expr) => { !$ self . config . file_lines () . is_all () && !$ self . config . file_lines () . intersects (&$ self . psess . lookup_line_range ($ span)) } ; }
};
}
