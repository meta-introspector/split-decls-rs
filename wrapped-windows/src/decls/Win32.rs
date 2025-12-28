macro_rules! Win32 {
    () => {
        # [cfg (feature = "Win32")] mod Win32 ;
    };
}

Win32!()