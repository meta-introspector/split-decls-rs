// Generated macro for get_ancestry (function)
macro_rules! Depcrate_ancestryget_ancestry {
() => {
// Module: crate::ancestry
// Provides: {"get_ancestry"}
// Dependencies: {}
# [doc = " Determines the ancestry of an actual span or event."] # [doc = ""] # [doc = " The rules for determining the ancestry are as follows:"] # [doc = ""] # [doc = " +------------+--------------+-----------------+---------------------+"] # [doc = " | Contextual | Current Span | Explicit Parent | Ancestry            |"] # [doc = " +------------+--------------+-----------------+---------------------+"] # [doc = " | Yes        | Yes          | -               | HasContextualParent |"] # [doc = " | Yes        | No           | -               | IsContextualRoot    |"] # [doc = " | No         | -            | Yes             | HasExplicitParent   |"] # [doc = " | No         | -            | No              | IsExplicitRoot      |"] # [doc = " +------------+--------------+-----------------+---------------------+"] pub (crate) fn get_ancestry (item : impl HasAncestry , lookup_current : impl FnOnce () -> Option < span :: Id > , actual_span : impl FnOnce (& span :: Id) -> Option < ActualSpan > ,) -> ActualAncestry { if item . is_contextual () { if let Some (parent_id) = lookup_current () { let contextual_parent_span = actual_span (& parent_id) . expect ("tracing-mock: contextual parent cannot \
                            be looked up by ID. Was it recorded correctly?" ,) ; ActualAncestry :: HasContextualParent (contextual_parent_span) } else { ActualAncestry :: IsContextualRoot } } else if item . is_root () { ActualAncestry :: IsExplicitRoot } else { let parent_id = item . parent () . expect ("tracing-mock: is_contextual=false is_root=false \
                        but no explicit parent found. This is a bug!" ,) ; let explicit_parent_span = actual_span (parent_id) . expect ("tracing-mock: explicit parent cannot be looked \
                        up by ID. Is the provided Span ID valid: {parent_id}" ,) ; ActualAncestry :: HasExplicitParent (explicit_parent_span) } }
};
}
