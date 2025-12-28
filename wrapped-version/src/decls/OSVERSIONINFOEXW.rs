macro_rules! OSVERSIONINFOEXW {
    () => {
        # [repr (C)] # [derive (Clone , Copy)] pub struct OSVERSIONINFOEXW { pub dwOSVersionInfoSize : u32 , pub dwMajorVersion : u32 , pub dwMinorVersion : u32 , pub dwBuildNumber : u32 , pub dwPlatformId : u32 , pub szCSDVersion : [u16 ; 128] , pub wServicePackMajor : u16 , pub wServicePackMinor : u16 , pub wSuiteMask : u16 , pub wProductType : u8 , pub wReserved : u8 , }
    };
}

OSVERSIONINFOEXW!()