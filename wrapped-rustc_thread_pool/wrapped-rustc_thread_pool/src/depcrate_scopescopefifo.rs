// Generated macro for ScopeFifo (struct)
macro_rules! Depcrate_scopeScopeFifo {
() => {
// Module: crate::scope
// Provides: {"ScopeFifo"}
// Dependencies: {}
# [doc = " Represents a fork-join scope which can be used to spawn any number of tasks."] # [doc = " Those spawned from the same thread are prioritized in relative FIFO order."] # [doc = " See [`scope_fifo()`] for more information."] # [doc = ""] # [doc = "[`scope_fifo()`]: fn.scope_fifo.html"] pub struct ScopeFifo < 'scope > { base : ScopeBase < 'scope > , fifos : Vec < JobFifo > , }
};
}
