// Generated macro for parse_os_version (function)
macro_rules! Depcrate_sys_platform_version_darwinparse_os_version {
() => {
// Module: crate::sys::platform_version::darwin
// Provides: {"parse_os_version"}
// Dependencies: {}
# [doc = " Parse an OS version from a bytestring like b\"10.1\" or b\"14.3.7\"."] fn parse_os_version (version : & [u8]) -> Result < OSVersion , ParseIntError > { if let Some ((major , minor)) = version . split_once (| & b | b == b'.') { let major = u16 :: from_ascii (major) ? ; if let Some ((minor , patch)) = minor . split_once (| & b | b == b'.') { let minor = u8 :: from_ascii (minor) ? ; let patch = u8 :: from_ascii (patch) ? ; Ok (pack_os_version (major , minor , patch)) } else { let minor = u8 :: from_ascii (minor) ? ; Ok (pack_os_version (major , minor , 0)) } } else { let major = u16 :: from_ascii (version) ? ; Ok (pack_os_version (major , 0 , 0)) } }
};
}
