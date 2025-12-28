macro_rules! DebugVisualizerInvalid {
    () => {
        # [derive (Diagnostic)] # [diag (passes_debug_visualizer_invalid)] # [note (passes_note_1)] # [note (passes_note_2)] # [note (passes_note_3)] pub (crate) struct DebugVisualizerInvalid { # [primary_span] pub span : Span , }
    };
}

DebugVisualizerInvalid!()