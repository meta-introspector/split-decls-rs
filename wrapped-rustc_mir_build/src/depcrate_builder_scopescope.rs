// Generated macro for Scope (struct)
macro_rules! Depcrate_builder_scopeScope {
() => {
// Module: crate::builder::scope
// Provides: {"Scope"}
// Dependencies: {}
# [derive (Debug)] struct Scope { # [doc = " The source scope this scope was created in."] source_scope : SourceScope , # [doc = " the region span of this scope within source code."] region_scope : region :: Scope , # [doc = " set of places to drop when exiting this scope. This starts"] # [doc = " out empty but grows as variables are declared during the"] # [doc = " building process. This is a stack, so we always drop from the"] # [doc = " end of the vector (top of the stack) first."] drops : Vec < DropData > , moved_locals : Vec < Local > , # [doc = " The drop index that will drop everything in and below this scope on an"] # [doc = " unwind path."] cached_unwind_block : Option < DropIdx > , # [doc = " The drop index that will drop everything in and below this scope on a"] # [doc = " coroutine drop path."] cached_coroutine_drop_block : Option < DropIdx > , }
};
}
