// Generated macro for impl_33 (impl)
macro_rules! Depcrate_errorimpl_33 {
() => {
// Module: crate::error
// Provides: {"impl_33"}
// Dependencies: {}
impl < T , E > InstrumentResult < T > for Result < T , E > where E : InstrumentError , { type Instrumented = < E as InstrumentError > :: Instrumented ; fn in_current_span (self) -> Result < T , Self :: Instrumented > { self . map_err (E :: in_current_span) } }
};
}
