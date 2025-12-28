macro_rules! DebugVisualizerPlacement {
    () => {
        # [derive (Diagnostic)] # [diag (passes_debug_visualizer_placement)] pub (crate) struct DebugVisualizerPlacement { # [primary_span] pub span : Span , }
    };
}

DebugVisualizerPlacement!()