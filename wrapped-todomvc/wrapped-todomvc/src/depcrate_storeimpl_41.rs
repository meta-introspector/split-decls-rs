// Generated macro for impl_41 (impl)
macro_rules! Depcrate_storeimpl_41 {
() => {
// Module: crate::store
// Provides: {"impl_41"}
// Dependencies: {}
impl Item { pub fn update (& mut self , update : & ItemUpdate) { match update { ItemUpdate :: Title { title , .. } => { self . title = title . to_string () ; } ItemUpdate :: Completed { completed , .. } => { self . completed = * completed ; } } } }
};
}
