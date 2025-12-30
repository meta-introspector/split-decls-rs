// Generated macro for impl_25 (impl)
macro_rules! Depcrate_ioimpl_25 {
() => {
// Module: crate::io
// Provides: {"impl_25"}
// Dependencies: {}
# [doc = " Ensures that Mock isn't dropped with data \"inside\"."] impl Drop for Mock { fn drop (& mut self) { if std :: thread :: panicking () { return ; } self . inner . actions . iter () . for_each (| a | match a { Action :: Read (data) => assert ! (data . is_empty () , "There is still data left to read. {}" , self . pmsg ()) , Action :: Write (data) => assert ! (data . is_empty () , "There is still data left to write. {}" , self . pmsg ()) , _ => () , }) ; } }
};
}
