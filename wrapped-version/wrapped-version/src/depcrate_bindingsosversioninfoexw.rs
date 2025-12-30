// Generated macro for OSVERSIONINFOEXW (struct)
macro_rules! Depcrate_bindingsOSVERSIONINFOEXW {
() => {
// Module: crate::bindings
// Provides: {"OSVERSIONINFOEXW"}
// Dependencies: {}
# [repr (C)] # [derive (Clone , Copy)] pub struct OSVERSIONINFOEXW { pub dwOSVersionInfoSize : u32 , pub dwMajorVersion : u32 , pub dwMinorVersion : u32 , pub dwBuildNumber : u32 , pub dwPlatformId : u32 , pub szCSDVersion : [u16 ; 128] , pub wServicePackMajor : u16 , pub wServicePackMinor : u16 , pub wSuiteMask : u16 , pub wProductType : u8 , pub wReserved : u8 , }
};
}
