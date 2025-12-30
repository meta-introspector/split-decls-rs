// Generated macro for Filestat (struct)
macro_rules! Depcrate_lib_generatedFilestat {
() => {
// Module: crate::lib_generated
// Provides: {"Filestat"}
// Dependencies: {}
# [repr (C)] # [derive (Copy , Clone , Debug)] pub struct Filestat { # [doc = " Device ID of device containing the file."] pub dev : Device , # [doc = " File serial number."] pub ino : Inode , # [doc = " File type."] pub filetype : Filetype , # [doc = " Number of hard links to the file."] pub nlink : Linkcount , # [doc = " For regular files, the file size in bytes. For symbolic links, the length in bytes of the pathname contained in the symbolic link."] pub size : Filesize , # [doc = " Last data access timestamp."] pub atim : Timestamp , # [doc = " Last data modification timestamp."] pub mtim : Timestamp , # [doc = " Last file status change timestamp."] pub ctim : Timestamp , }
};
}
