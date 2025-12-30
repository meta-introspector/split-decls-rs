// Generated macro for macro_1 (macro)
macro_rules! Depcratemacro_1 {
() => {
// Module: crate
// Provides: {"macro_1"}
// Dependencies: {}
# [cfg (all (any (sha2_backend = "riscv-zknh" , sha2_backend = "riscv-zknh-compact") , not (any (any (target_arch = "riscv32" , target_arch = "riscv64")))))] compile_error ! ("The Zknh backends can be enabled only for RISC-V targets") ;
};
}
