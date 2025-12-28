macro_rules! macro_352 {
    () => {
        bitflags :: bitflags ! { # [doc = " `IORING_CQE_F_*` flags for use with [`io_uring_cqe`]."] # [repr (transparent)] # [derive (Default , Copy , Clone , Eq , PartialEq , Hash , Debug)] pub struct IoringCqeFlags : u32 { # [doc = " `IORING_CQE_F_BUFFER`"] const BUFFER = bitcast ! (sys :: IORING_CQE_F_BUFFER) ; # [doc = " `IORING_CQE_F_MORE`"] const MORE = bitcast ! (sys :: IORING_CQE_F_MORE) ; # [doc = " `IORING_CQE_F_SOCK_NONEMPTY`"] const SOCK_NONEMPTY = bitcast ! (sys :: IORING_CQE_F_SOCK_NONEMPTY) ; # [doc = " `IORING_CQE_F_NOTIF`"] const NOTIF = bitcast ! (sys :: IORING_CQE_F_NOTIF) ; # [doc = " <https://docs.rs/bitflags/*/bitflags/#externally-defined-flags>"] const _ = ! 0 ; } }
    };
}

macro_352!()