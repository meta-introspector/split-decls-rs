// Generated macro for impl_53 (impl)
macro_rules! Depcrate_storeimpl_53 {
() => {
// Module: crate::store
// Provides: {"impl_53"}
// Dependencies: {}
impl ItemQuery { fn matches (& self , item : & Item) -> bool { match * self { ItemQuery :: EmptyItemQuery => true , ItemQuery :: Id { ref id } => & item . id == id , ItemQuery :: Completed { completed } => item . completed == completed , } } }
};
}
