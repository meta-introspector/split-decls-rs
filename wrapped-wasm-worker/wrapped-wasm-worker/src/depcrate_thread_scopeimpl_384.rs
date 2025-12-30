// Generated macro for impl_384 (impl)
macro_rules! Depcrate_thread_scopeimpl_384 {
() => {
// Module: crate::thread::scope
// Provides: {"impl_384"}
// Dependencies: {}
impl < F , T > Debug for State < '_ , '_ , F , T > { fn fmt (& self , formatter : & mut Formatter < '_ >) -> fmt :: Result { match self { Self :: Task { scope , .. } => formatter . debug_struct ("Task") . field ("task" , & any :: type_name :: < F > ()) . field ("scope" , & scope) . finish () , Self :: Wait { scope , .. } => formatter . debug_struct ("Wait") . field ("result" , & any :: type_name :: < T > ()) . field ("scope" , & scope) . finish () , Self :: None => formatter . write_str ("None") , } } }
};
}
