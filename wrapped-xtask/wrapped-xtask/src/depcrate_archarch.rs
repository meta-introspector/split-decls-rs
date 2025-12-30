// Generated macro for Arch (enum)
macro_rules! Depcrate_archArch {
() => {
// Module: crate::arch
// Provides: {"Arch"}
// Dependencies: {}
# [doc = " Target architecture."] # [derive (ValueEnum , Clone , Copy , PartialEq , Eq , Debug)] # [value (rename_all = "snake_case")] pub enum Arch { # [doc = " x86-64"] X86_64 , # [doc = " AArch64"] Aarch64 , # [doc = " AArch64, big-endian"] Aarch64Be , # [doc = " 64-bit RISC-V"] Riscv64 , }
};
}
