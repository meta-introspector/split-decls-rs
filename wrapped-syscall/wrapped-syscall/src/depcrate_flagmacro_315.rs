// Generated macro for macro_315 (macro)
macro_rules! Depcrate_flagmacro_315 {
() => {
// Module: crate::flag
// Provides: {"macro_315"}
// Dependencies: {}
bitflags ! { pub struct PtraceFlags : u64 { # [doc = " Stop before a syscall is handled. Send PTRACE_FLAG_IGNORE to not"] # [doc = " handle the syscall."] const PTRACE_STOP_PRE_SYSCALL = 0x0000_0000_0000_0001 ; # [doc = " Stop after a syscall is handled."] const PTRACE_STOP_POST_SYSCALL = 0x0000_0000_0000_0002 ; # [doc = " Stop after exactly one instruction. TODO: This may not handle"] # [doc = " fexec/signal boundaries. Should it?"] const PTRACE_STOP_SINGLESTEP = 0x0000_0000_0000_0004 ; # [doc = " Stop before a signal is handled. Send PTRACE_FLAG_IGNORE to not"] # [doc = " handle signal."] const PTRACE_STOP_SIGNAL = 0x0000_0000_0000_0008 ; # [doc = " Stop on a software breakpoint, such as the int3 instruction for"] # [doc = " x86_64."] const PTRACE_STOP_BREAKPOINT = 0x0000_0000_0000_0010 ; # [doc = " Stop just before exiting for good."] const PTRACE_STOP_EXIT = 0x0000_0000_0000_0020 ; const PTRACE_STOP_MASK = 0x0000_0000_0000_00FF ; # [doc = " Sent when a child is cloned, giving you the opportunity to trace it."] # [doc = " If you don't catch this, the child is started as normal."] const PTRACE_EVENT_CLONE = 0x0000_0000_0000_0100 ; # [doc = " Sent when current-addrspace is changed, allowing the tracer to reopen the memory file."] const PTRACE_EVENT_ADDRSPACE_SWITCH = 0x0000_0000_0000_0200 ; const PTRACE_EVENT_MASK = 0x0000_0000_0000_0F00 ; # [doc = " Special meaning, depending on the event. Usually, when fired before"] # [doc = " an action, it will skip performing that action."] const PTRACE_FLAG_IGNORE = 0x0000_0000_0000_1000 ; const PTRACE_FLAG_MASK = 0x0000_0000_0000_F000 ; } }
};
}
