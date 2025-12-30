// Generated macro for check_cfgs (function)
macro_rules! Depcrate_palcheck_cfgs {
() => {
// Module: crate::pal
// Provides: {"check_cfgs"}
// Dependencies: {}
fn check_cfgs (contents : & str , file : & Path , bad : & mut bool , saw_target_arch : & mut bool , saw_cfg_bang : & mut bool ,) { let cfgs = parse_cfgs (contents) ; let mut line_numbers : Option < Vec < usize > > = None ; let mut err = | idx : usize , cfg : & str | { if line_numbers . is_none () { line_numbers = Some (contents . match_indices ('\n') . map (| (i , _) | i) . collect ()) ; } let line_numbers = line_numbers . as_ref () . expect ("") ; let line = match line_numbers . binary_search (& idx) { Ok (_) => unreachable ! () , Err (i) => i + 1 , } ; tidy_error ! (bad , "{}:{}: platform-specific cfg: {}" , file . display () , line , cfg) ; } ; for (idx , cfg) in cfgs { if ! * saw_target_arch && cfg . contains ("target_arch") { * saw_target_arch = true } if ! * saw_cfg_bang && cfg . contains ("cfg!") { * saw_cfg_bang = true } let contains_platform_specific_cfg = cfg . contains ("target_os") || cfg . contains ("target_env") || cfg . contains ("target_abi") || cfg . contains ("target_vendor") || cfg . contains ("target_family") || cfg . contains ("unix") || cfg . contains ("windows") ; if ! contains_platform_specific_cfg { continue ; } let preceded_by_doc_comment = { let pre_contents = & contents [.. idx] ; let pre_newline = pre_contents . rfind ('\n') ; let pre_doc_comment = pre_contents . rfind ("///") ; match (pre_newline , pre_doc_comment) { (Some (n) , Some (c)) => n < c , (None , Some (_)) => true , (_ , None) => false , } } ; if preceded_by_doc_comment { continue ; } if cfg . contains ("test") { continue ; } err (idx , cfg) ; } }
};
}
