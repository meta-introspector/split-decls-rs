// Generated macro for Cursor (struct)
macro_rules! Depcrate_cursorCursor {
() => {
// Module: crate::cursor
// Provides: {"Cursor"}
// Dependencies: {}
# [doc = " A cursor into an input `TokenStream`'s data. This cursor holds a reference"] # [doc = " into the immutable data which is used internally to represent a"] # [doc = " `TokenStream`, and can be efficiently manipulated and copied around."] # [doc = ""] # [doc = " An empty `Cursor` can be created directly, or one may create a `SynomBuffer`"] # [doc = " object and get a cursor to its first token with `begin()`."] # [doc = ""] # [doc = " Two cursors are equal if they have the same location in the same input"] # [doc = " stream, and have the same scope."] # [derive (Copy , Clone , Eq , PartialEq)] pub struct Cursor < 'a > { # [doc = " The current entry which the `Cursor` is pointing at."] ptr : * const Entry , # [doc = " This is the only `Entry::End(..)` object which this cursor is allowed to"] # [doc = " point at. All other `End` objects are skipped over in `Cursor::create`."] scope : * const Entry , # [doc = " This uses the &'a reference which guarantees that these pointers are"] # [doc = " still valid."] marker : PhantomData < & 'a Entry > , }
};
}
