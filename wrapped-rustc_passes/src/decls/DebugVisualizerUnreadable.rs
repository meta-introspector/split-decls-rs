macro_rules! DebugVisualizerUnreadable {
    () => {
        # [derive (Diagnostic)] # [diag (passes_debug_visualizer_unreadable)] pub (crate) struct DebugVisualizerUnreadable < 'a > { # [primary_span] pub span : Span , pub file : & 'a Path , pub error : Error , }
    };
}

DebugVisualizerUnreadable!()