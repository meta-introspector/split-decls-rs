// Generated macro for other_31803 (other)
macro_rules! Depcrate_um_memoryapiother_31803 {
() => {
// Module: crate::um::memoryapi
// Provides: {"other_31803"}
// Dependencies: {}
extern "system" { pub fn RegisterBadMemoryNotification (Callback : PBAD_MEMORY_CALLBACK_ROUTINE ,) -> PVOID ; pub fn UnregisterBadMemoryNotification (RegistrationHandle : PVOID ,) -> BOOL ; }
};
}
