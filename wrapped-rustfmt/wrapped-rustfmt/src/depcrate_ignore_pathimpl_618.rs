// Generated macro for impl_618 (impl)
macro_rules! Depcrate_ignore_pathimpl_618 {
() => {
// Module: crate::ignore_path
// Provides: {"impl_618"}
// Dependencies: {}
impl IgnorePathSet { pub (crate) fn from_ignore_list (ignore_list : & IgnoreList) -> Result < Self , ignore :: Error > { let mut ignore_builder = gitignore :: GitignoreBuilder :: new (ignore_list . rustfmt_toml_path ()) ; for ignore_path in ignore_list { ignore_builder . add_line (None , ignore_path . to_str () . unwrap ()) ? ; } Ok (IgnorePathSet { ignore_set : ignore_builder . build () ? , }) } pub (crate) fn is_match (& self , file_name : & FileName) -> bool { match file_name { FileName :: Stdin => false , FileName :: Real (p) => self . ignore_set . matched_path_or_any_parents (p , false) . is_ignore () , } } }
};
}
