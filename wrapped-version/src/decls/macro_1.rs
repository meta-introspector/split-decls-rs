macro_rules! deps {
    () => {
        OSVERSIONINFOW!();
        NTSTATUS!();
    };
}

macro_rules! macro_1 {
    () => {
        deps!();
        windows_link :: link ! ("ntdll.dll" "system" fn RtlGetVersion (lpversioninformation : * mut OSVERSIONINFOW) -> NTSTATUS) ;
    };
}

macro_1!();