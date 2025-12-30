// Generated macro for get_frequencies (function)
macro_rules! Depcrate_windows_cpuget_frequencies {
() => {
// Module: crate::windows::cpu
// Provides: {"get_frequencies"}
// Dependencies: {}
pub (crate) fn get_frequencies (nb_cpus : usize) -> Vec < u64 > { let size = nb_cpus * mem :: size_of :: < PROCESSOR_POWER_INFORMATION > () ; let mut infos : Vec < PROCESSOR_POWER_INFORMATION > = Vec :: with_capacity (nb_cpus) ; unsafe { if CallNtPowerInformation (ProcessorInformation , None , 0 , Some (infos . as_mut_ptr () as _) , size as _ ,) . is_ok () { infos . set_len (nb_cpus) ; return infos . into_iter () . map (| i | i . CurrentMhz as u64) . collect :: < Vec < _ > > () ; } } sysinfo_debug ! ("get_frequencies: CallNtPowerInformation failed") ; vec ! [0 ; nb_cpus] }
};
}
