// Generated macro for screen_buffer_info (function)
macro_rules! Depcrate_consolescreen_buffer_info {
() => {
// Module: crate::console
// Provides: {"screen_buffer_info"}
// Dependencies: {}
# [doc = " Query the given handle for information about the console's screen buffer."] # [doc = ""] # [doc = " The given handle should represent a console. Otherwise, an error is"] # [doc = " returned."] # [doc = ""] # [doc = " This corresponds to calling [`GetConsoleScreenBufferInfo`]."] # [doc = ""] # [doc = " [`GetConsoleScreenBufferInfo`]: https://docs.microsoft.com/en-us/windows/console/getconsolescreenbufferinfo"] pub fn screen_buffer_info < H : AsHandleRef > (h : H ,) -> io :: Result < ScreenBufferInfo > { unsafe { let mut info : CONSOLE_SCREEN_BUFFER_INFO = mem :: zeroed () ; let rc = GetConsoleScreenBufferInfo (h . as_raw () as HANDLE , & mut info) ; if rc == 0 { return Err (io :: Error :: last_os_error ()) ; } Ok (ScreenBufferInfo (info)) } }
};
}
