macro_rules! other_4 {
    () => {
        # [cfg (not (feature = "windows_raw_dylib"))] # [cfg (not (target_os = "cygwin"))] # [cfg_attr (target_vendor = "win7" , link (name = "advapi32"))] # [link (name = "ntdll")] # [link (name = "userenv")] # [link (name = "ws2_32")] # [link (name = "dbghelp")] unsafe extern "C" { }
    };
}

other_4!();