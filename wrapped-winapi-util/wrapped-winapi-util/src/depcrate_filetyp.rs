// Generated macro for typ (function)
macro_rules! Depcrate_filetyp {
() => {
// Module: crate::file
// Provides: {"typ"}
// Dependencies: {}
# [doc = " Returns the file type of the given handle."] # [doc = ""] # [doc = " If there was a problem querying the file type, then an error is returned."] # [doc = ""] # [doc = " This corresponds to calling [`GetFileType`]."] # [doc = ""] # [doc = " [`GetFileType`]: https://docs.microsoft.com/en-us/windows/desktop/api/fileapi/nf-fileapi-getfiletype"] pub fn typ < H : AsHandleRef > (h : H) -> io :: Result < Type > { unsafe { let rc = GetFileType (h . as_raw () as HANDLE) ; if rc == 0 && GetLastError () != NO_ERROR { return Err (io :: Error :: last_os_error ()) ; } Ok (Type (rc)) } }
};
}
