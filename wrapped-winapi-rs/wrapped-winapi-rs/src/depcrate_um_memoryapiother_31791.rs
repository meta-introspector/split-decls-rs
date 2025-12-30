// Generated macro for other_31791 (other)
macro_rules! Depcrate_um_memoryapiother_31791 {
() => {
// Module: crate::um::memoryapi
// Provides: {"other_31791"}
// Dependencies: {}
extern "system" { pub fn CreateMemoryResourceNotification (NotificationType : MEMORY_RESOURCE_NOTIFICATION_TYPE ,) -> HANDLE ; pub fn QueryMemoryResourceNotification (ResourceNotificationHandle : HANDLE , ResourceState : PBOOL ,) -> BOOL ; }
};
}
