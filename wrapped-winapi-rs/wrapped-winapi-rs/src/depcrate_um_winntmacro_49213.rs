// Generated macro for macro_49213 (macro)
macro_rules! Depcrate_um_winntmacro_49213 {
() => {
// Module: crate::um::winnt
// Provides: {"macro_49213"}
// Dependencies: {}
# [cfg (target_arch = "arm")] IFDEF ! { pub const ARM_MAX_BREAKPOINTS : usize = 8 ; pub const ARM_MAX_WATCHPOINTS : usize = 1 ; STRUCT ! { struct NEON128 { Low : ULONGLONG , High : LONGLONG , } } pub type PNEON128 = * mut NEON128 ; UNION ! { union CONTEXT_u { [u64 ; 32] , Q Q_mut : [NEON128 ; 16] , D D_mut : [ULONGLONG ; 32] , S S_mut : [DWORD ; 32] , } } STRUCT ! { struct CONTEXT { ContextFlags : DWORD , R0 : DWORD , R1 : DWORD , R2 : DWORD , R3 : DWORD , R4 : DWORD , R5 : DWORD , R6 : DWORD , R7 : DWORD , R8 : DWORD , R9 : DWORD , R10 : DWORD , R11 : DWORD , R12 : DWORD , Sp : DWORD , Lr : DWORD , Pc : DWORD , Cpsr : DWORD , Fpsrc : DWORD , Padding : DWORD , u : CONTEXT_u , Bvr : [DWORD ; ARM_MAX_BREAKPOINTS] , Bcr : [DWORD ; ARM_MAX_BREAKPOINTS] , Wvr : [DWORD ; ARM_MAX_WATCHPOINTS] , Wcr : [DWORD ; ARM_MAX_WATCHPOINTS] , Padding2 : [DWORD ; 2] , } } pub type PCONTEXT = * mut CONTEXT ; }
};
}
