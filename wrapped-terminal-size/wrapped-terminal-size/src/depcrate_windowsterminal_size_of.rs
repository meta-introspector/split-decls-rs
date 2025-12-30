// Generated macro for terminal_size_of (function)
macro_rules! Depcrate_windowsterminal_size_of {
() => {
// Module: crate::windows
// Provides: {"terminal_size_of"}
// Dependencies: {}
# [doc = " Returns the size of the terminal using the given handle, if available."] # [doc = ""] # [doc = " If the given handle is not a tty, returns `None`"] pub fn terminal_size_of < Handle : AsHandle > (handle : Handle) -> Option < (Width , Height) > { use windows_sys :: Win32 :: Foundation :: INVALID_HANDLE_VALUE ; use windows_sys :: Win32 :: System :: Console :: { GetConsoleScreenBufferInfo , CONSOLE_SCREEN_BUFFER_INFO , COORD , SMALL_RECT , } ; let hand = handle . as_handle () . as_raw_handle () as windows_sys :: Win32 :: Foundation :: HANDLE ; if hand == INVALID_HANDLE_VALUE { return None ; } let zc = COORD { X : 0 , Y : 0 } ; let mut csbi = CONSOLE_SCREEN_BUFFER_INFO { dwSize : zc , dwCursorPosition : zc , wAttributes : 0 , srWindow : SMALL_RECT { Left : 0 , Top : 0 , Right : 0 , Bottom : 0 , } , dwMaximumWindowSize : zc , } ; if unsafe { GetConsoleScreenBufferInfo (hand , & mut csbi) } == 0 { return None ; } let w : Width = Width ((csbi . srWindow . Right - csbi . srWindow . Left + 1) as u16) ; let h : Height = Height ((csbi . srWindow . Bottom - csbi . srWindow . Top + 1) as u16) ; Some ((w , h)) }
};
}
