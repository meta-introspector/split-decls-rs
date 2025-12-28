macro_rules! macro_8 {
    () => {
        windows_link :: link ! ("kernel32.dll" "system" fn Sleep (dwmilliseconds : u32)) ;
    };
}

macro_8!()