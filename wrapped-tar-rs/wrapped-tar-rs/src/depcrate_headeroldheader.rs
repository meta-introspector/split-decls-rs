// Generated macro for OldHeader (struct)
macro_rules! Depcrate_headerOldHeader {
() => {
// Module: crate::header
// Provides: {"OldHeader"}
// Dependencies: {}
# [doc = " Representation of the header of an entry in an archive"] # [repr (C)] # [allow (missing_docs)] pub struct OldHeader { pub name : [u8 ; 100] , pub mode : [u8 ; 8] , pub uid : [u8 ; 8] , pub gid : [u8 ; 8] , pub size : [u8 ; 12] , pub mtime : [u8 ; 12] , pub cksum : [u8 ; 8] , pub linkflag : [u8 ; 1] , pub linkname : [u8 ; 100] , pub pad : [u8 ; 255] , }
};
}
