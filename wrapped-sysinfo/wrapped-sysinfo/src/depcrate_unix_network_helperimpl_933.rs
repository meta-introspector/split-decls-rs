// Generated macro for impl_933 (impl)
macro_rules! Depcrate_unix_network_helperimpl_933 {
() => {
// Module: crate::unix::network_helper
// Provides: {"impl_933"}
// Dependencies: {}
# [cfg (any (target_os = "macos" , target_os = "freebsd" , target_os = "ios"))] impl From < & libc :: sockaddr_dl > for MacAddr { fn from (value : & libc :: sockaddr_dl) -> Self { let sdl_data = value . sdl_data ; let sdl_nlen = value . sdl_nlen as usize ; if sdl_nlen + 5 < 12 { MacAddr ([sdl_data [sdl_nlen] as u8 , sdl_data [sdl_nlen + 1] as u8 , sdl_data [sdl_nlen + 2] as u8 , sdl_data [sdl_nlen + 3] as u8 , sdl_data [sdl_nlen + 4] as u8 , sdl_data [sdl_nlen + 5] as u8 ,]) } else { MacAddr :: UNSPECIFIED } } }
};
}
