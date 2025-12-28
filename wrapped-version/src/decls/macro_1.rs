macro_rules! deps {
    () => {
        NTSTATUS!();
        OSVERSIONINFOW!();
    };
}

macro_rules! macro_1 {
    () => {
        deps!();
        windows_link :: link ! ("ntdll.dll" "system" fn RtlGetVersion (lpversioninformation : * mut OSVERSIONINFOW) -> NTSTATUS) ;
    };
}

macro_1!()