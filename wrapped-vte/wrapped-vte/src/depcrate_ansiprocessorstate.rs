// Generated macro for ProcessorState (struct)
macro_rules! Depcrate_ansiProcessorState {
() => {
// Module: crate::ansi
// Provides: {"ProcessorState"}
// Dependencies: {}
# [doc = " Internal state for VTE processor."] # [derive (Debug , Default)] struct ProcessorState < T : Timeout > { # [doc = " Last processed character for repetition."] preceding_char : Option < char > , # [doc = " State for synchronized terminal updates."] sync_state : SyncState < T > , }
};
}
