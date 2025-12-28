macro_rules! OsVersion {
    () => {
        # [doc = " Operating system version information."] # [derive (Debug , Copy , Clone , PartialEq , Eq , PartialOrd , Ord)] pub struct OsVersion { # [doc = " The major version number of the operating system."] pub major : u32 , # [doc = " The minor version number of the operating system."] pub minor : u32 , # [doc = " The major version number of the latest service pack installed on the system."] pub pack : u32 , # [doc = " The build number of the operating system."] pub build : u32 , }
    };
}

OsVersion!();