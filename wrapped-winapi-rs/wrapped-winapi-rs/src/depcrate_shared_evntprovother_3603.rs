// Generated macro for other_3603 (other)
macro_rules! Depcrate_shared_evntprovother_3603 {
() => {
// Module: crate::shared::evntprov
// Provides: {"other_3603"}
// Dependencies: {}
extern "system" { pub fn EventRegister (ProviderId : LPCGUID , EnableCallback : PENABLECALLBACK , CallbackContext : PVOID , RegHandle : PREGHANDLE ,) -> ULONG ; pub fn EventUnregister (RegHandle : REGHANDLE ,) -> ULONG ; pub fn EventSetInformation (RegHandle : REGHANDLE , InformationClass : EVENT_INFO_CLASS , EventInformation : PVOID , InformationLength : ULONG ,) -> ULONG ; pub fn EventEnabled (RegHandle : REGHANDLE , EventDescriptor : PCEVENT_DESCRIPTOR ,) -> BOOLEAN ; pub fn EventProviderEnabled (RegHandle : REGHANDLE , Level : UCHAR , Keyword : ULONGLONG ,) -> BOOLEAN ; pub fn EventWrite (RegHandle : REGHANDLE , EventDescriptor : PCEVENT_DESCRIPTOR , UserDataCount : ULONG , UserData : PEVENT_DATA_DESCRIPTOR ,) -> ULONG ; pub fn EventWriteTransfer (RegHandle : REGHANDLE , EventDescriptor : PCEVENT_DESCRIPTOR , ActivityId : LPCGUID , RelatedActivityId : LPCGUID , UserDataCount : ULONG , UserData : PEVENT_DATA_DESCRIPTOR ,) -> ULONG ; pub fn EventWriteEx (RegHandle : REGHANDLE , EventDescriptor : PCEVENT_DESCRIPTOR , Filter : ULONG64 , Flags : ULONG , ActivityId : LPCGUID , RelatedActivityId : LPCGUID , UserDataCount : ULONG , UserData : PEVENT_DATA_DESCRIPTOR ,) -> ULONG ; pub fn EventWriteString (RegHandle : REGHANDLE , Level : UCHAR , Keyword : ULONGLONG , EventString : PCWSTR ,) -> ULONG ; pub fn EventActivityIdControl (ControlCode : ULONG , ActivityId : LPGUID ,) -> ULONG ; }
};
}
