macro_rules! macro_5 {
    () => {
        windows_link :: link ! ("kernel32.dll" "system" fn GetCurrentThreadId () -> u32) ;
    };
}

macro_5!();