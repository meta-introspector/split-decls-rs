// Generated macro for UstarHeader (struct)
macro_rules! Depcrate_headerUstarHeader {
() => {
// Module: crate::header
// Provides: {"UstarHeader"}
// Dependencies: {}
# [doc = " Representation of the header of an entry in an archive"] # [repr (C)] # [allow (missing_docs)] pub struct UstarHeader { pub name : [u8 ; 100] , pub mode : [u8 ; 8] , pub uid : [u8 ; 8] , pub gid : [u8 ; 8] , pub size : [u8 ; 12] , pub mtime : [u8 ; 12] , pub cksum : [u8 ; 8] , pub typeflag : [u8 ; 1] , pub linkname : [u8 ; 100] , pub magic : [u8 ; 6] , pub version : [u8 ; 2] , pub uname : [u8 ; 32] , pub gname : [u8 ; 32] , pub dev_major : [u8 ; 8] , pub dev_minor : [u8 ; 8] , pub prefix : [u8 ; 155] , pub pad : [u8 ; 12] , }
};
}
