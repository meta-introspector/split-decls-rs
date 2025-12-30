// Generated macro for GnuHeader (struct)
macro_rules! Depcrate_headerGnuHeader {
() => {
// Module: crate::header
// Provides: {"GnuHeader"}
// Dependencies: {}
# [doc = " Representation of the header of an entry in an archive"] # [repr (C)] # [allow (missing_docs)] pub struct GnuHeader { pub name : [u8 ; 100] , pub mode : [u8 ; 8] , pub uid : [u8 ; 8] , pub gid : [u8 ; 8] , pub size : [u8 ; 12] , pub mtime : [u8 ; 12] , pub cksum : [u8 ; 8] , pub typeflag : [u8 ; 1] , pub linkname : [u8 ; 100] , pub magic : [u8 ; 6] , pub version : [u8 ; 2] , pub uname : [u8 ; 32] , pub gname : [u8 ; 32] , pub dev_major : [u8 ; 8] , pub dev_minor : [u8 ; 8] , pub atime : [u8 ; 12] , pub ctime : [u8 ; 12] , pub offset : [u8 ; 12] , pub longnames : [u8 ; 4] , pub unused : [u8 ; 1] , pub sparse : [GnuSparseHeader ; GNU_SPARSE_HEADERS_COUNT] , pub isextended : [u8 ; 1] , pub realsize : [u8 ; 12] , pub pad : [u8 ; 17] , }
};
}
