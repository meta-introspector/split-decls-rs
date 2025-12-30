// Generated macro for keydata (module)
macro_rules! Depcrate_unix_apple_macos_ffikeydata {
() => {
// Module: crate::unix::apple::macos::ffi
// Provides: {"keydata"}
// Dependencies: {}
# [cfg (all (not (feature = "apple-sandbox") , all (feature = "component" , any (target_arch = "x86" , target_arch = "x86_64")) ,))] mod keydata { # [cfg_attr (feature = "debug" , derive (Eq , Hash , PartialEq))] # [derive (Clone)] # [repr (C)] pub struct Val_t { pub key : [i8 ; 5] , pub data_size : u32 , pub data_type : [i8 ; 5] , pub bytes : [i8 ; 32] , } # [cfg_attr (feature = "debug" , derive (Debug , Eq , Hash , PartialEq))] # [repr (C)] pub struct KeyData_vers_t { pub major : u8 , pub minor : u8 , pub build : u8 , pub reserved : [u8 ; 1] , pub release : u16 , } # [cfg_attr (feature = "debug" , derive (Debug , Eq , Hash , PartialEq))] # [repr (C)] pub struct KeyData_pLimitData_t { pub version : u16 , pub length : u16 , pub cpu_plimit : u32 , pub gpu_plimit : u32 , pub mem_plimit : u32 , } # [cfg_attr (feature = "debug" , derive (Debug , Eq , Hash , PartialEq))] # [repr (C)] pub struct KeyData_keyInfo_t { pub data_size : u32 , pub data_type : u32 , pub data_attributes : u8 , } # [cfg_attr (feature = "debug" , derive (Debug , Eq , Hash , PartialEq))] # [repr (C)] pub struct KeyData_t { pub key : u32 , pub vers : KeyData_vers_t , pub p_limit_data : KeyData_pLimitData_t , pub key_info : KeyData_keyInfo_t , pub result : u8 , pub status : u8 , pub data8 : u8 , pub data32 : u32 , pub bytes : [i8 ; 32] , } # [allow (dead_code)] pub const KERNEL_INDEX_SMC : i32 = 2 ; # [allow (dead_code)] pub const SMC_CMD_READ_KEYINFO : u8 = 9 ; # [allow (dead_code)] pub const SMC_CMD_READ_BYTES : u8 = 5 ; }
};
}
