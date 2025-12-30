// Generated macro for impl_66 (impl)
macro_rules! Depcrate_wycheproofimpl_66 {
() => {
// Module: crate::wycheproof
// Provides: {"impl_66"}
// Dependencies: {}
impl std :: fmt :: Display for CaseResult { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { write ! (f , "{}" , match self { CaseResult :: Valid => "valid" , CaseResult :: Invalid => "invalid" , CaseResult :: Acceptable => "acceptable" , }) } }
};
}
