// Generated macro for impl_451 (impl)
macro_rules! Depcrate_schemev2impl_451 {
() => {
// Module: crate::schemev2
// Provides: {"impl_451"}
// Dependencies: {}
impl Opcode { pub fn try_from_raw (raw : u8) -> Option < Self > { use Opcode :: * ; Some (match raw { 0 => Open , 1 => Rmdir , 2 => Unlink , 3 => Close , 4 => Dup , 5 => Read , 6 => Write , 7 => Fsize , 8 => Fchmod , 9 => Fchown , 10 => Fcntl , 11 => Fevent , 12 => Sendfd , 13 => Fpath , 14 => Frename , 15 => Fstat , 16 => Fstatvfs , 17 => Fsync , 18 => Ftruncate , 19 => Futimens , 20 => MmapPrep , 21 => RequestMmap , 22 => Mremap , 23 => Munmap , 24 => Msync , 25 => Cancel , 26 => Getdents , 27 => CloseMsg , 28 => Call , 29 => OpenAt , 30 => Flink , 31 => Recvfd , _ => return None , }) } }
};
}
