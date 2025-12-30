// Generated macro for mode (function)
macro_rules! Depcrate_consolemode {
() => {
// Module: crate::console
// Provides: {"mode"}
// Dependencies: {}
# [doc = " Query the mode of the console represented by the given handle."] # [doc = ""] # [doc = " This corresponds to calling [`GetConsoleMode`], which describes the return"] # [doc = " value."] # [doc = ""] # [doc = " [`GetConsoleMode`]: https://docs.microsoft.com/en-us/windows/console/getconsolemode"] pub fn mode < H : AsHandleRef > (h : H) -> io :: Result < u32 > { let mut mode = 0 ; if unsafe { GetConsoleMode (h . as_raw () as HANDLE , & mut mode) } == 0 { Err (io :: Error :: last_os_error ()) } else { Ok (mode) } }
};
}
