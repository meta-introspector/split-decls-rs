// Generated macro for get_vendor_id_not_great (function)
macro_rules! Depcrate_windows_cpuget_vendor_id_not_great {
() => {
// Module: crate::windows::cpu
// Provides: {"get_vendor_id_not_great"}
// Dependencies: {}
fn get_vendor_id_not_great (info : & SYSTEM_INFO) -> String { unsafe { match info . Anonymous . Anonymous . wProcessorArchitecture { SystemInformation :: PROCESSOR_ARCHITECTURE_INTEL => "Intel x86" , SystemInformation :: PROCESSOR_ARCHITECTURE_MIPS => "MIPS" , SystemInformation :: PROCESSOR_ARCHITECTURE_ALPHA => "RISC Alpha" , SystemInformation :: PROCESSOR_ARCHITECTURE_PPC => "PPC" , SystemInformation :: PROCESSOR_ARCHITECTURE_SHX => "SHX" , SystemInformation :: PROCESSOR_ARCHITECTURE_ARM => "ARM" , SystemInformation :: PROCESSOR_ARCHITECTURE_IA64 => "Intel Itanium-based x64" , SystemInformation :: PROCESSOR_ARCHITECTURE_ALPHA64 => "RISC Alpha x64" , SystemInformation :: PROCESSOR_ARCHITECTURE_MSIL => "MSIL" , SystemInformation :: PROCESSOR_ARCHITECTURE_AMD64 => "(Intel or AMD) x64" , SystemInformation :: PROCESSOR_ARCHITECTURE_IA32_ON_WIN64 => "Intel Itanium-based x86" , SystemInformation :: PROCESSOR_ARCHITECTURE_NEUTRAL => "unknown" , SystemInformation :: PROCESSOR_ARCHITECTURE_ARM64 => "ARM x64" , SystemInformation :: PROCESSOR_ARCHITECTURE_ARM32_ON_WIN64 => "ARM" , SystemInformation :: PROCESSOR_ARCHITECTURE_IA32_ON_ARM64 => "Intel Itanium-based x86" , _ => "unknown" , } . to_owned () } }
};
}
