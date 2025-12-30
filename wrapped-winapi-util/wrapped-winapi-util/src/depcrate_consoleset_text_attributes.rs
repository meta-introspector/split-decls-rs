// Generated macro for set_text_attributes (function)
macro_rules! Depcrate_consoleset_text_attributes {
() => {
// Module: crate::console
// Provides: {"set_text_attributes"}
// Dependencies: {}
# [doc = " Set the text attributes of the console represented by the given handle."] # [doc = ""] # [doc = " This corresponds to calling [`SetConsoleTextAttribute`]."] # [doc = ""] # [doc = " [`SetConsoleTextAttribute`]: https://docs.microsoft.com/en-us/windows/console/setconsoletextattribute"] pub fn set_text_attributes < H : AsHandleRef > (h : H , attributes : u16 ,) -> io :: Result < () > { if unsafe { SetConsoleTextAttribute (h . as_raw () as HANDLE , attributes) } == 0 { Err (io :: Error :: last_os_error ()) } else { Ok (()) } }
};
}
