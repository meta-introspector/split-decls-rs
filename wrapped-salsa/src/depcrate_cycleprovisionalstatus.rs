// Generated macro for ProvisionalStatus (enum)
macro_rules! Depcrate_cycleProvisionalStatus {
() => {
// Module: crate::cycle
// Provides: {"ProvisionalStatus"}
// Dependencies: {}
# [derive (Debug)] pub enum ProvisionalStatus < 'db > { Provisional { iteration : IterationCount , verified_at : Revision , cycle_heads : & 'db CycleHeads , } , Final { iteration : IterationCount , verified_at : Revision , } , FallbackImmediate , }
};
}
