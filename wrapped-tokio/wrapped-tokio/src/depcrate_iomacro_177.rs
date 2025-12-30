// Generated macro for macro_177 (macro)
macro_rules! Depcrate_iomacro_177 {
() => {
// Module: crate::io
// Provides: {"macro_177"}
// Dependencies: {}
cfg_io_blocking ! { # [doc = " Types in this module can be mocked out in tests."] mod sys { pub (crate) use crate :: blocking :: spawn_blocking as run ; pub (crate) use crate :: blocking :: JoinHandle as Blocking ; } }
};
}
