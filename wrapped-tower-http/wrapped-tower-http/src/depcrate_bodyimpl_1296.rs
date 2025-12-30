// Generated macro for impl_1296 (impl)
macro_rules! Depcrate_bodyimpl_1296 {
() => {
// Module: crate::body
// Provides: {"impl_1296"}
// Dependencies: {}
impl < B > Body for Limited < B > where B : Body , B :: Error : Into < BoxError > , { type Data = B :: Data ; type Error = BoxError ; body_methods ! () ; }
};
}
