// Generated macro for impl_26 (impl)
macro_rules! Depcrate_syn_utilsimpl_26 {
() => {
// Module: crate::syn_utils
// Provides: {"impl_26"}
// Dependencies: {}
impl std :: fmt :: Display for RawFieldKey { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { match self { Self :: Named (name) => name . fmt (f) , Self :: Unnamed (idx) => idx . fmt (f) , } } }
};
}
