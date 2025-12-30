// Generated macro for Opcode (enum)
macro_rules! Depcrate_schemev2Opcode {
() => {
// Module: crate::schemev2
// Provides: {"Opcode"}
// Dependencies: {}
# [repr (u8)] # [non_exhaustive] # [derive (Clone , Copy , Debug)] pub enum Opcode { Open = 0 , Rmdir = 1 , Unlink = 2 , Close = 3 , Dup = 4 , Read = 5 , Write = 6 , Fsize = 7 , Fchmod = 8 , Fchown = 9 , Fcntl = 10 , Fevent = 11 , Sendfd = 12 , Fpath = 13 , Frename = 14 , Fstat = 15 , Fstatvfs = 16 , Fsync = 17 , Ftruncate = 18 , Futimens = 19 , MmapPrep = 20 , RequestMmap = 21 , Mremap = 22 , Munmap = 23 , Msync = 24 , Cancel = 25 , Getdents = 26 , CloseMsg = 27 , Call = 28 , OpenAt = 29 , Flink = 30 , Recvfd = 31 , }
};
}
