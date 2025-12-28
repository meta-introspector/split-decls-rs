macro_rules! deps {
    () => {
        PCSTR!();
        HKEY!();
        REG_ROUTINE_FLAGS!();
        REG_VALUE_TYPE!();
        WIN32_ERROR!();
    };
}

macro_rules! macro_0 {
    () => {
        deps!();
        windows_link :: link ! ("advapi32.dll" "system" fn RegGetValueA (hkey : HKEY , lpsubkey : PCSTR , lpvalue : PCSTR , dwflags : REG_ROUTINE_FLAGS , pdwtype : * mut REG_VALUE_TYPE , pvdata : * mut core :: ffi :: c_void , pcbdata : * mut u32) -> WIN32_ERROR) ;
    };
}

macro_0!()