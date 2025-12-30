// Generated macro for other_34250 (other)
macro_rules! Depcrate_um_perflibother_34250 {
() => {
// Module: crate::um::perflib
// Provides: {"other_34250"}
// Dependencies: {}
extern "system" { pub fn PerfEnumerateCounterSet (szMachine : LPCWSTR , pCounterSetIds : LPGUID , cCounterSetIds : DWORD , pcCounterSetIdsActual : LPDWORD ,) -> ULONG ; pub fn PerfEnumerateCounterSetInstances (szMachine : LPCWSTR , pCounterSetIds : LPCGUID , pInstances : PPERF_INSTANCE_HEADER , cbInstances : DWORD , pcbInstancesActual : LPDWORD ,) -> ULONG ; pub fn PerfQueryCounterSetRegistrationInfo (szMachine : LPCWSTR , pCounterSetId : LPCGUID , requestCode : PerfRegInfoType , requestLangId : DWORD , pbRegInfo : LPBYTE , cbRegInfo : DWORD , pcbRegInfoActual : LPDWORD ,) -> ULONG ; pub fn PerfOpenQueryHandle (szMachine : LPCWSTR , hQuery : * mut HANDLE ,) -> ULONG ; pub fn PerfCloseQueryHandle (hQuery : HANDLE ,) -> ULONG ; pub fn PerfQueryCounterInfo (hQuery : HANDLE , pCounters : PPERF_COUNTER_IDENTIFIER , cbCounters : DWORD , pcbCountersActual : LPDWORD ,) -> ULONG ; pub fn PerfQueryCounterData (hQuery : HANDLE , pCounterBlock : PPERF_DATA_HEADER , cbCounterBlock : DWORD , pcbCounterBlockActual : LPDWORD ,) -> ULONG ; pub fn PerfAddCounters (hQuery : HANDLE , pCounters : PPERF_COUNTER_IDENTIFIER , cbCounters : DWORD ,) -> ULONG ; pub fn PerfDeleteCounters (hQuery : HANDLE , pCounters : PPERF_COUNTER_IDENTIFIER , cbCounters : DWORD ,) -> ULONG ; }
};
}
