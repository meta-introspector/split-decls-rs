// Generated macro for macro_5 (macro)
macro_rules! Depcratemacro_5 {
() => {
// Module: crate
// Provides: {"macro_5"}
// Dependencies: {}
cfg_if ! { if # [cfg (target_arch = "wasm32")] { pub mod wasm ; use wasm :: disassemble_myself ; } else { mod disassembly ; use crate :: disassembly :: disassemble_myself ; } }
};
}
