// Generated macro for impl_1576 (impl)
macro_rules! Depcrateimpl_1576 {
() => {
// Module: crate
// Provides: {"impl_1576"}
// Dependencies: {}
impl Input { fn file_name (& self) -> FileName { match * self { Input :: File (ref file) => FileName :: Real (file . clone ()) , Input :: Text (..) => FileName :: Stdin , } } fn to_directory_ownership (& self) -> Option < DirectoryOwnership > { match self { Input :: File (ref file) => { let file_stem = file . file_stem () ? ; if file . parent () ? . to_path_buf () . join (file_stem) . is_dir () { Some (DirectoryOwnership :: Owned { relative : file_stem . to_str () . map (symbol :: Ident :: from_str) , }) } else { None } } _ => None , } } }
};
}
