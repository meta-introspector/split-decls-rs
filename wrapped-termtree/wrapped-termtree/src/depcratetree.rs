// Generated macro for Tree (struct)
macro_rules! DepcrateTree {
() => {
// Module: crate
// Provides: {"Tree"}
// Dependencies: {}
# [doc = " a simple recursive type which is able to render its"] # [doc = " components in a tree-like format"] # [derive (Debug , Clone)] pub struct Tree < D : Display > { pub root : D , pub leaves : Vec < Tree < D > > , multiline : bool , glyphs : Option < GlyphPalette > , }
};
}
