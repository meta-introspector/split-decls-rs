// Generated macro for Sqe (struct)
macro_rules! Depcrate_schemev2Sqe {
() => {
// Module: crate::schemev2
// Provides: {"Sqe"}
// Dependencies: {}
# [repr (C)] # [derive (Clone , Copy , Debug , Default)] pub struct Sqe { pub opcode : u8 , pub sqe_flags : SqeFlags , pub _rsvd : u16 , pub tag : u32 , pub args : [u64 ; 6] , pub caller : u64 , }
};
}
