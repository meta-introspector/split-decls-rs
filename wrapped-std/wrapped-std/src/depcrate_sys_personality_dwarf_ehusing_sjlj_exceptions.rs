// Generated macro for USING_SJLJ_EXCEPTIONS (const)
macro_rules! Depcrate_sys_personality_dwarf_ehUSING_SJLJ_EXCEPTIONS {
() => {
// Module: crate::sys::personality::dwarf::eh
// Provides: {"USING_SJLJ_EXCEPTIONS"}
// Dependencies: {}
# [doc = " 32-bit ARM Darwin platforms uses SjLj exceptions."] # [doc = ""] # [doc = " The exception is watchOS armv7k (specifically that subarchitecture), which"] # [doc = " instead uses DWARF Call Frame Information (CFI) unwinding."] # [doc = ""] # [doc = " <https://github.com/llvm/llvm-project/blob/llvmorg-18.1.4/clang/lib/Driver/ToolChains/Darwin.cpp#L3107-L3119>"] pub const USING_SJLJ_EXCEPTIONS : bool = cfg ! (all (target_vendor = "apple" , not (target_os = "watchos") , target_arch = "arm")) ;
};
}
