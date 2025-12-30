// Generated macro for CCompilerKind (enum)
macro_rules! Depcrate_compiler_cCCompilerKind {
() => {
// Module: crate::compiler::c
// Provides: {"CCompilerKind"}
// Dependencies: {}
# [doc = " Supported C compilers."] # [derive (Debug , PartialEq , Eq , Clone)] pub enum CCompilerKind { # [doc = " GCC"] Gcc , # [doc = " clang"] Clang , # [doc = " Diab"] Diab , # [doc = " Microsoft Visual C++"] Msvc , # [doc = " NVIDIA CUDA compiler"] Nvcc , # [doc = " NVIDIA CUDA front-end"] CudaFE , # [doc = " NVIDIA CUDA optimizer and PTX generator"] Cicc , # [doc = " NVIDIA CUDA PTX assembler"] Ptxas , # [doc = " NVIDIA hpc c, c++ compiler"] Nvhpc , # [doc = " Tasking VX"] TaskingVX , }
};
}
