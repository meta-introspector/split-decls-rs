// Generated macro for files_modified_batch_filter (function)
macro_rules! Depcratefiles_modified_batch_filter {
() => {
// Module: crate
// Provides: {"files_modified_batch_filter"}
// Dependencies: {}
# [doc = " Similar to `files_modified`, but only involves a single call to `git`."] # [doc = ""] # [doc = " removes all elements from `items` that do not cause any match when `pred` is called with the list of modifed files."] # [doc = ""] # [doc = " if in CI, no elements will be removed."] pub fn files_modified_batch_filter < T > (ci_info : & CiInfo , items : & mut Vec < T > , pred : impl Fn (& T , & str) -> bool ,) { if CiEnv :: is_ci () { return ; } let Some (base_commit) = & ci_info . base_commit else { eprintln ! ("No base commit, assuming all files are modified") ; return ; } ; match crate :: git_diff (base_commit , "--name-status") { Some (output) => { let modified_files : Vec < _ > = output . lines () . filter_map (| ln | { let (status , name) = ln . trim_end () . split_once ('\t') . expect ("bad format from `git diff --name-status`") ; if status == "M" { Some (name) } else { None } }) . collect () ; items . retain (| item | { for modified_file in & modified_files { if pred (item , modified_file) { return true ; } } false }) ; } None => { eprintln ! ("warning: failed to run `git diff` to check for changes") ; eprintln ! ("warning: assuming all files are modified") ; } } }
};
}
