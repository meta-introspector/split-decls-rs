// Generated macro for impl_175 (impl)
macro_rules! Depcrate_sftpimpl_175 {
() => {
// Module: crate::sftp
// Provides: {"impl_175"}
// Dependencies: {}
impl Seek for File { # [doc = " Move the file handle's internal pointer to an arbitrary location."] # [doc = ""] # [doc = " libssh2 implements file pointers as a localized concept to make file"] # [doc = " access appear more POSIX like. No packets are exchanged with the server"] # [doc = " during a seek operation. The localized file pointer is simply used as a"] # [doc = " convenience offset during read/write operations."] # [doc = ""] # [doc = " You MUST NOT seek during writing or reading a file with SFTP, as the"] # [doc = " internals use outstanding packets and changing the \"file position\""] # [doc = " during transit will results in badness."] fn seek (& mut self , how : SeekFrom) -> io :: Result < u64 > { let next = match how { SeekFrom :: Start (pos) => pos , SeekFrom :: Current (offset) => { let locked = self . lock () ? ; let cur = unsafe { raw :: libssh2_sftp_tell64 (locked . raw) } ; (cur as i64 + offset) as u64 } SeekFrom :: End (offset) => match self . stat () { Ok (s) => match s . size { Some (size) => (size as i64 + offset) as u64 , None => return Err (io :: Error :: new (ErrorKind :: Other , "no file size available")) , } , Err (e) => return Err (io :: Error :: new (ErrorKind :: Other , e)) , } , } ; let locked = self . lock () ? ; unsafe { raw :: libssh2_sftp_seek64 (locked . raw , next) } Ok (next) } }
};
}
