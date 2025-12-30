// Generated macro for IN4_UNALIGNED_ADDR_EQUAL (function)
macro_rules! Depcrate_shared_mstcpipIN4_UNALIGNED_ADDR_EQUAL {
() => {
// Module: crate::shared::mstcpip
// Provides: {"IN4_UNALIGNED_ADDR_EQUAL"}
// Dependencies: {}
# [inline] pub fn IN4_UNALIGNED_ADDR_EQUAL (a : & IN_ADDR , b : & IN_ADDR) -> bool { unsafe { * a . S_un . S_addr () == * b . S_un . S_addr () } }
};
}
