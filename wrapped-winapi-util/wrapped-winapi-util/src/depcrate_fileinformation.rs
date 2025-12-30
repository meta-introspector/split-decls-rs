// Generated macro for information (function)
macro_rules! Depcrate_fileinformation {
() => {
// Module: crate::file
// Provides: {"information"}
// Dependencies: {}
# [doc = " Return various pieces of information about a file."] # [doc = ""] # [doc = " This includes information such as a file's size, unique identifier and"] # [doc = " time related fields."] # [doc = ""] # [doc = " This corresponds to calling [`GetFileInformationByHandle`]."] # [doc = ""] # [doc = " [`GetFileInformationByHandle`]: https://docs.microsoft.com/en-us/windows/desktop/api/fileapi/nf-fileapi-getfileinformationbyhandle"] pub fn information < H : AsHandleRef > (h : H) -> io :: Result < Information > { unsafe { let mut info : BY_HANDLE_FILE_INFORMATION = mem :: zeroed () ; let rc = GetFileInformationByHandle (h . as_raw () as HANDLE , & mut info) ; if rc == 0 { return Err (io :: Error :: last_os_error ()) ; } ; Ok (Information (info)) } }
};
}
