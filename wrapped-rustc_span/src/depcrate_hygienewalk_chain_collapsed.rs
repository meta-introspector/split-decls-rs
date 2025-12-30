// Generated macro for walk_chain_collapsed (function)
macro_rules! Depcrate_hygienewalk_chain_collapsed {
() => {
// Module: crate::hygiene
// Provides: {"walk_chain_collapsed"}
// Dependencies: {}
# [doc = " In order to have good line stepping behavior in debugger, for the given span we return its"] # [doc = " outermost macro call site that still has a `#[collapse_debuginfo(yes)]` property on it."] # [doc = " We also stop walking call sites at the function body level because no line stepping can occur"] # [doc = " at the level above that."] # [doc = " The returned span can then be used in emitted debuginfo."] pub fn walk_chain_collapsed (span : Span , to : Span) -> Span { HygieneData :: with (| data | data . walk_chain_collapsed (span , to)) }
};
}
