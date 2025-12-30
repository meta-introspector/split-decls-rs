// Generated macro for attr_prefix (function)
macro_rules! Depcrate_attrattr_prefix {
() => {
// Module: crate::attr
// Provides: {"attr_prefix"}
// Dependencies: {}
fn attr_prefix (attr : & ast :: Attribute) -> & 'static str { match attr . style { ast :: AttrStyle :: Inner => "#!" , ast :: AttrStyle :: Outer => "#" , } }
};
}
