macro_rules! VersionInfo {
    () => {
        pub struct VersionInfo { pub major : u8 , pub minor : u8 , pub patch : u16 , pub host_compiler : Option < String > , pub commit_hash : Option < String > , pub commit_date : Option < String > , pub crate_name : String , }
    };
}

VersionInfo!()