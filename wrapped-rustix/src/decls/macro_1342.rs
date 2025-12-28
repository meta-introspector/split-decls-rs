macro_rules! macro_1342 {
    () => {
        bitflags ! { # [doc = " Zero means addresses that are passed for the purpose of being"] # [doc = " dereferenced by the kernel must be untagged."] # [repr (transparent)] # [derive (Copy , Clone , Eq , PartialEq , Hash , Debug)] pub struct TaggedAddressMode : u32 { # [doc = " Addresses that are passed for the purpose of being dereferenced by"] # [doc = " the kernel may be tagged."] const ENABLED = 1_u32 << 0 ; # [doc = " Synchronous tag check fault mode."] const TCF_SYNC = 1_u32 << 1 ; # [doc = " Asynchronous tag check fault mode."] const TCF_ASYNC = 1_u32 << 2 ; # [doc = " <https://docs.rs/bitflags/*/bitflags/#externally-defined-flags>"] const _ = ! 0 ; } }
    };
}

macro_1342!()