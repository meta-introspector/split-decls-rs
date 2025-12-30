// Generated macro for SubModKind (enum)
macro_rules! Depcrate_modulesSubModKind {
() => {
// Module: crate::modules
// Provides: {"SubModKind"}
// Dependencies: {}
# [derive (Clone)] enum SubModKind < 'a , 'ast > { # [doc = " `mod foo;`"] External (PathBuf , DirectoryOwnership , Module < 'ast >) , # [doc = " `mod foo;` with multiple sources."] MultiExternal (Vec < (PathBuf , DirectoryOwnership , Module < 'ast >) >) , # [doc = " `mod foo {}`"] Internal (& 'a ast :: Item) , }
};
}
