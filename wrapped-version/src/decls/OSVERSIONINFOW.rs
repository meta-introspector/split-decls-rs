macro_rules! OSVERSIONINFOW {
    () => {
        # [repr (C)] # [derive (Clone , Copy)] pub struct OSVERSIONINFOW { pub dwOSVersionInfoSize : u32 , pub dwMajorVersion : u32 , pub dwMinorVersion : u32 , pub dwBuildNumber : u32 , pub dwPlatformId : u32 , pub szCSDVersion : [u16 ; 128] , }
    };
}

OSVERSIONINFOW!();