// Generated macro for Scopes (struct)
macro_rules! Depcrate_builder_scopeScopes {
() => {
// Module: crate::builder::scope
// Provides: {"Scopes"}
// Dependencies: {}
# [derive (Debug)] pub (crate) struct Scopes < 'tcx > { scopes : Vec < Scope > , # [doc = " The current set of breakable scopes. See module comment for more details."] breakable_scopes : Vec < BreakableScope < 'tcx > > , const_continuable_scopes : Vec < ConstContinuableScope < 'tcx > > , # [doc = " The scope of the innermost if-then currently being lowered."] if_then_scope : Option < IfThenScope > , # [doc = " Drops that need to be done on unwind paths. See the comment on"] # [doc = " [DropTree] for more details."] unwind_drops : DropTree , # [doc = " Drops that need to be done on paths to the `CoroutineDrop` terminator."] coroutine_drops : DropTree , }
};
}
