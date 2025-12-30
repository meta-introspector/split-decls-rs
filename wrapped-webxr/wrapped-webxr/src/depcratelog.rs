// Generated macro for log (macro)
macro_rules! Depcratelog {
() => {
// Module: crate
// Provides: {"log"}
// Dependencies: {}
macro_rules ! log { ($ ($ t : tt) *) => { web_sys :: console :: log_1 (& format ! ($ ($ t) *) . into ()) ; } }
};
}
