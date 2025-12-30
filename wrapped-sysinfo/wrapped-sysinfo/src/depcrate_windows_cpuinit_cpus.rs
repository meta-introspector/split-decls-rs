// Generated macro for init_cpus (function)
macro_rules! Depcrate_windows_cpuinit_cpus {
() => {
// Module: crate::windows::cpu
// Provides: {"init_cpus"}
// Dependencies: {}
fn init_cpus (refresh_kind : CpuRefreshKind) -> Vec < Cpu > { unsafe { let mut sys_info = SYSTEM_INFO :: default () ; GetSystemInfo (& mut sys_info) ; let (vendor_id , brand) = get_vendor_id_and_brand (& sys_info) ; let nb_cpus = sys_info . dwNumberOfProcessors as usize ; let frequencies = if refresh_kind . frequency () { get_frequencies (nb_cpus) } else { vec ! [0 ; nb_cpus] } ; let mut ret = Vec :: with_capacity (nb_cpus + 1) ; for (nb , frequency) in frequencies . iter () . enumerate () { ret . push (Cpu { inner : CpuInner :: new_with_values (format ! ("CPU {}" , nb + 1) , vendor_id . clone () , brand . clone () , * frequency ,) , }) ; } ret } }
};
}
