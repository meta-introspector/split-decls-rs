// Generated macro for File (struct)
macro_rules! Depcrate_sftpFile {
() => {
// Module: crate::sftp
// Provides: {"File"}
// Dependencies: {}
# [doc = " A file handle to an SFTP connection."] # [doc = ""] # [doc = " Files behave similarly to `std::old_io::File` in that they are readable and"] # [doc = " writable and support operations like stat and seek."] # [doc = ""] # [doc = " Files are created through `open`, `create`, and `open_mode` on an instance"] # [doc = " of `Sftp`."] pub struct File { inner : Option < FileInner > , }
};
}
