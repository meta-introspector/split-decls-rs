// Generated macro for set_mode (function)
macro_rules! Depcrate_consoleset_mode {
() => {
// Module: crate::console
// Provides: {"set_mode"}
// Dependencies: {}
# [doc = " Set the mode of the console represented by the given handle."] # [doc = ""] # [doc = " This corresponds to calling [`SetConsoleMode`], which describes the format"] # [doc = " of the mode parameter."] # [doc = ""] # [doc = " [`SetConsoleMode`]: https://docs.microsoft.com/en-us/windows/console/setconsolemode"] pub fn set_mode < H : AsHandleRef > (h : H , mode : u32) -> io :: Result < () > { if unsafe { SetConsoleMode (h . as_raw () as HANDLE , mode) } == 0 { Err (io :: Error :: last_os_error ()) } else { Ok (()) } }
};
}
