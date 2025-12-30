// Generated macro for macro_75 (macro)
macro_rules! Depcrate_datamacro_75 {
() => {
// Module: crate::data
// Provides: {"macro_75"}
// Dependencies: {}
bitflags :: bitflags ! { # [derive (PartialEq , Eq , PartialOrd , Ord , Hash , Debug , Clone , Copy , Default)] pub struct GrantFlags : usize { const GRANT_READ = 0x0000_0001 ; const GRANT_WRITE = 0x0000_0002 ; const GRANT_EXEC = 0x0000_0004 ; const GRANT_SHARED = 0x0000_0008 ; const GRANT_LAZY = 0x0000_0010 ; const GRANT_SCHEME = 0x0000_0020 ; const GRANT_PHYS = 0x0000_0040 ; const GRANT_PINNED = 0x0000_0080 ; const GRANT_PHYS_CONTIGUOUS = 0x0000_0100 ; } }
};
}
