macro_rules! deps {
    () => {
        BSTR!();
        PCWSTR!();
    };
}

macro_rules! macro_84 {
    () => {
        deps!();
        windows_link :: link ! ("oleaut32.dll" "system" fn SysAllocStringLen (strin : PCWSTR , ui : u32) -> BSTR) ;
    };
}

macro_84!()