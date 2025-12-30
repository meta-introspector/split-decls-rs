// Generated macro for impl_127 (impl)
macro_rules! Depcrateimpl_127 {
() => {
// Module: crate
// Provides: {"impl_127"}
// Dependencies: {}
impl Iterator for IntoIter { type Item = Result < DirEntry > ; # [doc = " Advances the iterator and returns the next value."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " If the iterator fails to retrieve the next value, this method returns"] # [doc = " an error value. The error will be wrapped in an Option::Some."] fn next (& mut self) -> Option < Result < DirEntry > > { if let Some (start) = self . start . take () { if self . opts . same_file_system { let result = util :: device_num (& start) . map_err (| e | Error :: from_path (0 , start . clone () , e)) ; self . root_device = Some (itry ! (result)) ; } let dent = itry ! (DirEntry :: from_path (0 , start , false)) ; if let Some (result) = self . handle_entry (dent) { return Some (result) ; } } while ! self . stack_list . is_empty () { self . depth = self . stack_list . len () ; if let Some (dentry) = self . get_deferred_dir () { return Some (Ok (dentry)) ; } if self . depth > self . opts . max_depth { self . pop () ; continue ; } let next = self . stack_list . last_mut () . expect ("BUG: stack should be non-empty") . next () ; match next { None => self . pop () , Some (Err (err)) => return Some (Err (err)) , Some (Ok (dent)) => { if let Some (result) = self . handle_entry (dent) { return Some (result) ; } } } } if self . opts . contents_first { self . depth = self . stack_list . len () ; if let Some (dentry) = self . get_deferred_dir () { return Some (Ok (dentry)) ; } } None } }
};
}
