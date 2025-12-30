// Generated macro for impl_104 (impl)
macro_rules! Depcrate_serimpl_104 {
() => {
// Module: crate::ser
// Provides: {"impl_104"}
// Dependencies: {}
impl Error { fn top_level () -> Self { let msg = "top-level serializer supports only maps and structs" ; Error :: Custom (msg . into ()) } fn no_key () -> Self { let msg = "tried to serialize a value before serializing key" ; Error :: Custom (msg . into ()) } }
};
}
