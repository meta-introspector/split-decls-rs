// Generated macro for Replace (struct)
macro_rules! Depcrate_algorithms_replaceReplace {
() => {
// Module: crate::algorithms::replace
// Provides: {"Replace"}
// Dependencies: {}
# [doc = " A [`DiffHook`] that combines deletions and insertions to give blocks"] # [doc = " of maximal length, and replacements when appropriate."] # [doc = ""] # [doc = " It will replace [`DiffHook::insert`] and [`DiffHook::delete`] events when"] # [doc = " possible with [`DiffHook::replace`] events.  Note that even though the"] # [doc = " text processing in the crate does not use replace events and always resolves"] # [doc = " then back to delete and insert, it's useful to always use the replacer to"] # [doc = " ensure a consistent order of inserts and deletes.  This is why for instance"] # [doc = " the text diffing automatically uses this hook internally."] pub struct Replace < D : DiffHook > { d : D , del : Option < (usize , usize , usize) > , ins : Option < (usize , usize , usize) > , eq : Option < (usize , usize , usize) > , }
};
}
