// Generated macro for impl_1304 (impl)
macro_rules! Depcrate_mock_commandimpl_1304 {
() => {
// Module: crate::mock_command
// Provides: {"impl_1304"}
// Dependencies: {}
impl fmt :: Debug for ChildOrCall { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { ChildOrCall :: Child (ref r) => write ! (f , "ChildOrCall::Child({:?}" , r) , ChildOrCall :: Call (_) => write ! (f , "ChildOrCall::Call(...)") , } } }
};
}
