// Generated macro for impl_122 (impl)
macro_rules! Depcrateimpl_122 {
() => {
// Module: crate
// Provides: {"impl_122"}
// Dependencies: {}
impl IntoIterator for WalkDir { type Item = Result < DirEntry > ; type IntoIter = IntoIter ; fn into_iter (self) -> IntoIter { IntoIter { opts : self . opts , start : Some (self . root) , stack_list : vec ! [] , stack_path : vec ! [] , oldest_opened : 0 , depth : 0 , deferred_dirs : vec ! [] , root_device : None , } } }
};
}
