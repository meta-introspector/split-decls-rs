// Generated macro for Scope (struct)
macro_rules! Depcrate_registryScope {
() => {
// Module: crate::registry
// Provides: {"Scope"}
// Dependencies: {}
# [doc = " An iterator over the parents of a span, ordered from leaf to root."] # [doc = ""] # [doc = " This is returned by the [`SpanRef::scope`] method."] # [derive (Debug)] pub struct Scope < 'a , R > { registry : & 'a R , next : Option < Id > , # [cfg (all (feature = "registry" , feature = "std"))] filter : FilterId , }
};
}
