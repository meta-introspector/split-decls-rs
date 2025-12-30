// Generated macro for Dirent (struct)
macro_rules! Depcrate_lib_generatedDirent {
() => {
// Module: crate::lib_generated
// Provides: {"Dirent"}
// Dependencies: {}
# [repr (C)] # [derive (Copy , Clone , Debug)] pub struct Dirent { # [doc = " The offset of the next directory entry stored in this directory."] pub d_next : Dircookie , # [doc = " The serial number of the file referred to by this directory entry."] pub d_ino : Inode , # [doc = " The length of the name of the directory entry."] pub d_namlen : Dirnamlen , # [doc = " The type of the file referred to by this directory entry."] pub d_type : Filetype , }
};
}
