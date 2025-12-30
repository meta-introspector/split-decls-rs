// Generated macro for InitializeObjectAttributes (function)
macro_rules! Depcrate_shared_ntdefInitializeObjectAttributes {
() => {
// Module: crate::shared::ntdef
// Provides: {"InitializeObjectAttributes"}
// Dependencies: {}
# [inline] pub unsafe fn InitializeObjectAttributes (p : POBJECT_ATTRIBUTES , n : PUNICODE_STRING , a : ULONG , r : HANDLE , s : PVOID ,) { use core :: mem :: size_of ; (* p) . Length = size_of :: < OBJECT_ATTRIBUTES > () as ULONG ; (* p) . RootDirectory = r ; (* p) . Attributes = a ; (* p) . ObjectName = n ; (* p) . SecurityDescriptor = s ; (* p) . SecurityQualityOfService = NULL ; }
};
}
