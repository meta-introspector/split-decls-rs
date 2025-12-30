// Generated macro for StreamIsPartial (trait)
macro_rules! Depcrate_streamStreamIsPartial {
() => {
// Module: crate::stream
// Provides: {"StreamIsPartial"}
// Dependencies: {}
# [doc = " Marks the input as being the complete buffer or a partial buffer for streaming input"] # [doc = ""] # [doc = " See [`Partial`] for marking a presumed complete buffer type as a streaming buffer."] pub trait StreamIsPartial : Sized { # [doc = " Whether the stream is currently partial or complete"] type PartialState ; # [doc = " Mark the stream is complete"] # [must_use] fn complete (& mut self) -> Self :: PartialState ; # [doc = " Restore the stream back to its previous state"] fn restore_partial (& mut self , state : Self :: PartialState) ; # [doc = " Report whether the [`Stream`] is can ever be incomplete"] fn is_partial_supported () -> bool ; # [doc = " Report whether the [`Stream`] is currently incomplete"] # [inline (always)] fn is_partial (& self) -> bool { Self :: is_partial_supported () } }
};
}
