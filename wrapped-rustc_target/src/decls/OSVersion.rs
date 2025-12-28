macro_rules! OSVersion {
    () => {
        # [doc = " Deployment target or SDK version."] # [doc = ""] # [doc = " The size of the numbers in here are limited by Mach-O's `LC_BUILD_VERSION`."] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash , PartialOrd , Ord)] pub struct OSVersion { pub major : u16 , pub minor : u8 , pub patch : u8 , }
    };
}

OSVersion!();