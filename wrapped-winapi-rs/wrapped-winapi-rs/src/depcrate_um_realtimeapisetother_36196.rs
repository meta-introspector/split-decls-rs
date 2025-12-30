// Generated macro for other_36196 (other)
macro_rules! Depcrate_um_realtimeapisetother_36196 {
() => {
// Module: crate::um::realtimeapiset
// Provides: {"other_36196"}
// Dependencies: {}
extern "system" { pub fn QueryThreadCycleTime (ThreadHandle : HANDLE , CycleTime : PULONG64 ,) -> BOOL ; pub fn QueryProcessCycleTime (ProcessHandle : HANDLE , CycleTime : PULONG64 ,) -> BOOL ; pub fn QueryIdleProcessorCycleTime (BufferLength : PULONG , ProcessorIdleCycleTime : PULONG64 ,) -> BOOL ; pub fn QueryIdleProcessorCycleTimeEx (Group : USHORT , BufferLength : PULONG , ProcessorIdleCycleTime : PULONG64 ,) -> BOOL ; pub fn QueryUnbiasedInterruptTime (UnbiasedTime : PULONGLONG ,) -> BOOL ; }
};
}
