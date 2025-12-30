// Generated macro for arch_to_llvm_component (function)
macro_rules! Depcrate_target_specific_testsarch_to_llvm_component {
() => {
// Module: crate::target_specific_tests
// Provides: {"arch_to_llvm_component"}
// Dependencies: {}
fn arch_to_llvm_component (arch : & str) -> String { match arch { "amdgcn" => "amdgpu" . into () , "aarch64_be" | "arm64_32" | "arm64e" | "arm64ec" => "aarch64" . into () , "i386" | "i586" | "i686" | "x86" | "x86_64" | "x86_64h" => "x86" . into () , "loongarch32" | "loongarch64" => "loongarch" . into () , "nvptx64" => "nvptx" . into () , "s390x" => "systemz" . into () , "sparc64" | "sparcv9" => "sparc" . into () , "wasm32" | "wasm32v1" | "wasm64" => "webassembly" . into () , _ if arch . starts_with ("armeb") || arch . starts_with ("armv") || arch . starts_with ("thumbv") => { "arm" . into () } _ if arch . starts_with ("bpfe") => "bpf" . into () , _ if arch . starts_with ("mips") => "mips" . into () , _ if arch . starts_with ("powerpc") => "powerpc" . into () , _ if arch . starts_with ("riscv") => "riscv" . into () , _ => arch . to_ascii_lowercase () , } }
};
}
