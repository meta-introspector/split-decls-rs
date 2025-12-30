// Generated macro for Drive (trait)
macro_rules! Depcrate_util_call_all_commonDrive {
() => {
// Module: crate::util::call_all::common
// Provides: {"Drive"}
// Dependencies: {}
pub (crate) trait Drive < F : Future > { fn is_empty (& self) -> bool ; fn push (& mut self , future : F) ; fn poll (& mut self , cx : & mut Context < '_ >) -> Poll < Option < F :: Output > > ; }
};
}
