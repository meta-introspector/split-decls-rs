// Generated macro for BoxError (type)
macro_rules! Depcrate_resultBoxError {
() => {
// Module: crate::result
// Provides: {"BoxError"}
// Dependencies: {}
# [doc = " Generic thread-safe boxed error."] # [doc = ""] # [doc = " From all our prior experience we've learned that there is very little"] # [doc = " practical use in concrete error types. On the surface it seems appealing to"] # [doc = " use such errors, because they have, ahem, concrete type. But the flip side"] # [doc = " is that code in big projects quickly ends up being polluted with endless"] # [doc = " adapter error types to combine different APIs together, or, even worse, an"] # [doc = " Error god-object gets introduced to accommodate all possible error types."] # [doc = ""] # [doc = " On rare occasions concrete error types can be used, where handling of the"] # [doc = " error depends on the error kind. But, in practice, such cases are quite"] # [doc = " rare."] pub type BoxError = Box < dyn Error + Send + Sync + 'static > ;
};
}
