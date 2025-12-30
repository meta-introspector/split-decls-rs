// Generated macro for get_cpu_frequency (function)
macro_rules! Depcrate_unix_linux_cpuget_cpu_frequency {
() => {
// Module: crate::unix::linux::cpu
// Provides: {"get_cpu_frequency"}
// Dependencies: {}
pub (crate) fn get_cpu_frequency (cpu_core_index : usize) -> u64 { let mut s = String :: new () ; if File :: open (format ! ("/sys/devices/system/cpu/cpu{cpu_core_index}/cpufreq/scaling_cur_freq" ,)) . and_then (| mut f | f . read_to_string (& mut s)) . is_ok () { let freq_option = s . trim () . split ('\n') . next () ; if let Some (freq_string) = freq_option && let Ok (freq) = freq_string . parse :: < u64 > () { return freq / 1000 ; } } s . clear () ; if File :: open ("/proc/cpuinfo") . and_then (| mut f | f . read_to_string (& mut s)) . is_err () { return 0 ; } let find_cpu_mhz = s . split ('\n') . find (| line | { cpuinfo_is_key (line , b"cpu MHz\t") || cpuinfo_is_key (line , b"CPU MHz\t") || cpuinfo_is_key (line , b"BogoMIPS") || cpuinfo_is_key (line , b"clock\t") || cpuinfo_is_key (line , b"bogomips per cpu") }) ; find_cpu_mhz . and_then (| line | line . split (':') . next_back ()) . and_then (| val | val . replace ("MHz" , "") . trim () . parse :: < f64 > () . ok ()) . map (| speed | speed as u64) . unwrap_or_default () }
};
}
