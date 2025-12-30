// Generated macro for init_load_avg (function)
macro_rules! Depcrate_windows_cpuinit_load_avg {
() => {
// Module: crate::windows::cpu
// Provides: {"init_load_avg"}
// Dependencies: {}
unsafe fn init_load_avg () -> Mutex < Option < LoadAvg > > { let mut query = PDH_HQUERY :: default () ; unsafe { if PdhOpenQueryA (PCSTR :: null () , 0 , & mut query) != ERROR_SUCCESS . 0 { sysinfo_debug ! ("init_load_avg: PdhOpenQueryA failed") ; return Mutex :: new (None) ; } let counter = 0 ; if PdhAddEnglishCounterA (query , s ! ("\\System\\Cpu Queue Length") , 0 , & mut PDH_HCOUNTER (counter as * mut c_void) ,) != ERROR_SUCCESS . 0 { PdhCloseQuery (query) ; sysinfo_debug ! ("init_load_avg: failed to get CPU queue length") ; return Mutex :: new (None) ; } let event = match CreateEventA (None , false , false , s ! ("LoadUpdateEvent")) { Ok (ev) => ev , Err (_) => { PdhCloseQuery (query) ; sysinfo_debug ! ("init_load_avg: failed to create event `LoadUpdateEvent`") ; return Mutex :: new (None) ; } } ; if PdhCollectQueryDataEx (query , SAMPLING_INTERVAL as _ , event) != ERROR_SUCCESS . 0 { PdhCloseQuery (query) ; sysinfo_debug ! ("init_load_avg: PdhCollectQueryDataEx failed") ; return Mutex :: new (None) ; } let mut wait_handle = HANDLE :: default () ; if RegisterWaitForSingleObject (& mut wait_handle , event , Some (load_avg_callback) , Some (counter as * const c_void) , INFINITE , WT_EXECUTEDEFAULT ,) . is_ok () { Mutex :: new (Some (LoadAvg :: default ())) } else { PdhRemoveCounter (PDH_HCOUNTER (counter as * mut c_void)) ; PdhCloseQuery (query) ; sysinfo_debug ! ("init_load_avg: RegisterWaitForSingleObject failed") ; Mutex :: new (None) } } }
};
}
