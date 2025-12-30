// Generated macro for get_arm_implementer (function)
macro_rules! Depcrate_unix_linux_cpuget_arm_implementer {
() => {
// Module: crate::unix::linux::cpu
// Provides: {"get_arm_implementer"}
// Dependencies: {}
# [doc = " Obtain the implementer of this CPU core."] # [doc = ""] # [doc = " This has been obtained from util-linux's lscpu implementation, see"] # [doc = " https://github.com/util-linux/util-linux/blob/7076703b529d255600631306419cca1b48ab850a/sys-utils/lscpu-arm.c#L240"] # [doc = ""] # [doc = " This list will have to be updated every time a new vendor appears, please keep it synchronized"] # [doc = " with util-linux and update the link above with the commit you have used."] fn get_arm_implementer (implementer : u32) -> Option < & 'static str > { Some (match implementer { 0x41 => "ARM" , 0x42 => "Broadcom" , 0x43 => "Cavium" , 0x44 => "DEC" , 0x46 => "FUJITSU" , 0x48 => "HiSilicon" , 0x49 => "Infineon" , 0x4d => "Motorola/Freescale" , 0x4e => "NVIDIA" , 0x50 => "APM" , 0x51 => "Qualcomm" , 0x53 => "Samsung" , 0x56 => "Marvell" , 0x61 => "Apple" , 0x66 => "Faraday" , 0x69 => "Intel" , 0x70 => "Phytium" , 0xc0 => "Ampere" , _ => return None , }) }
};
}
