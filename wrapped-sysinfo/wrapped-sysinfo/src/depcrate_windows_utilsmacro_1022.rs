// Generated macro for macro_1022 (macro)
macro_rules! Depcrate_windows_utilsmacro_1022 {
() => {
// Module: crate::windows::utils
// Provides: {"macro_1022"}
// Dependencies: {}
cfg_if ! { if # [cfg (feature = "system")] { use windows :: Win32 :: System :: SystemInformation :: { FIRMWARE_TABLE_PROVIDER , GetSystemFirmwareTable } ; use super :: ffi :: SMBIOSType ; pub (crate) fn get_smbios_table () -> Option < Vec < u8 >> { const PROVIDER : FIRMWARE_TABLE_PROVIDER = FIRMWARE_TABLE_PROVIDER (u32 :: from_be_bytes (* b"RSMB")) ; let size = unsafe { GetSystemFirmwareTable (PROVIDER , 0 , None) } ; if size == 0 { return None ; } let mut buffer = vec ! [0u8 ; size as usize] ; let res = unsafe { GetSystemFirmwareTable (PROVIDER , 0 , Some (& mut buffer)) } ; if res == 0 { return None ; } Some (buffer) } pub (crate) fn parse_smbios < T : SMBIOSType > (table : & [u8] , number : u8) -> Option < (T , Vec <& str >) > { let mut found = false ; let mut i = 0 ; while i + 1 < table . len () { if table [i] == number { found = true ; break ; } i += table [i + 1] as usize ; while i < table . len () { if table [i] == 0 && table [i + 1] == 0 { i += 2 ; break ; } i += 1 ; } } if ! found { return None ; } let data = table . get (i ..) ?; if data . len () < std :: mem :: size_of ::< T > () { return None ; } let info : T = unsafe { std :: ptr :: read_unaligned (data . as_ptr () as * const _) } ; let values = table . get ((i + info . length () as usize) ..) . unwrap_or_default () . split (|& b | b == 0) . filter_map (| s | std :: str :: from_utf8 (s) . ok ()) . take_while (| s | ! s . is_empty ()) . collect () ; Some ((info , values)) } } }
};
}
