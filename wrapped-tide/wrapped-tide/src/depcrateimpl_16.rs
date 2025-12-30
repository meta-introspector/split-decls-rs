// Generated macro for impl_16 (impl)
macro_rules! Depcrateimpl_16 {
() => {
// Module: crate
// Provides: {"impl_16"}
// Dependencies: {}
impl < E : Executor > Clone for GraphQLEndpoint < E > { fn clone (& self) -> Self { Self { executor : self . executor . clone () , opts : self . opts , batch : self . batch , } } }
};
}
