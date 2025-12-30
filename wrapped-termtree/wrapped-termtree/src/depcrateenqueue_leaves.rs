// Generated macro for enqueue_leaves (function)
macro_rules! Depcrateenqueue_leaves {
() => {
// Module: crate
// Provides: {"enqueue_leaves"}
// Dependencies: {}
fn enqueue_leaves < 't , D : Display > (queue : & mut DisplauQueue < 't , D > , parent : & 't Tree < D > , parent_glyphs : & 't GlyphPalette , spaces : Rc < Vec < SpacePalette > > ,) { for (i , leaf) in parent . leaves . iter () . rev () . enumerate () { let last = i == 0 ; let glyphs = leaf . glyphs . as_ref () . unwrap_or (parent_glyphs) ; queue . push_front ((last , leaf , glyphs , spaces . clone ())) ; } }
};
}
