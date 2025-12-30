// Generated macro for ifmibdata (struct)
macro_rules! Depcrate_unix_apple_networkifmibdata {
() => {
// Module: crate::unix::apple::network
// Provides: {"ifmibdata"}
// Dependencies: {}
# [repr (C)] struct ifmibdata { ifmd_name : [c_char ; IFNAMSIZ] , ifmd_pcount : c_uint , ifmd_flags : c_uint , ifmd_snd_len : c_uint , ifmd_snd_maxlen : c_uint , ifmd_snd_drops : c_uint , ifmd_filler : [c_uint ; 4] , ifmd_data : if_data64 , }
};
}
