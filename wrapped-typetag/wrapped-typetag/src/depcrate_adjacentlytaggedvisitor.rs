// Generated macro for TaggedVisitor (struct)
macro_rules! Depcrate_adjacentlyTaggedVisitor {
() => {
// Module: crate::adjacently
// Provides: {"TaggedVisitor"}
// Dependencies: {}
struct TaggedVisitor < T : ? Sized + 'static > { trait_object : & 'static str , field_names : & 'static [& 'static str ; 2] , default_variant : Option < & 'static str > , registry : & 'static Registry < T > , deny_unknown_fields : bool , }
};
}
