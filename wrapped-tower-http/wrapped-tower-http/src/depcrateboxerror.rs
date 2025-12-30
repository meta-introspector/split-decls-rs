// Generated macro for BoxError (type)
macro_rules! DepcrateBoxError {
() => {
// Module: crate
// Provides: {"BoxError"}
// Dependencies: {}
# [doc = " Alias for a type-erased error type."] pub type BoxError = Box < dyn std :: error :: Error + Send + Sync > ;
};
}
