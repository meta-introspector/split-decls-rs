// Generated macro for TaggedVisitor (struct)
macro_rules! Depcrate_internallyTaggedVisitor {
() => {
// Module: crate::internally
// Provides: {"TaggedVisitor"}
// Dependencies: {}
struct TaggedVisitor < T : ? Sized + 'static > { trait_object : & 'static str , tag : & 'static str , default_variant : Option < & 'static str > , registry : & 'static Registry < T > , }
};
}
