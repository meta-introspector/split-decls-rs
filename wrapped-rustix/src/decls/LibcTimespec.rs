macro_rules! deps {
    () => {
        Secs!();
    };
}

macro_rules! LibcTimespec {
    () => {
        deps!();
        # [doc = " On 32-bit glibc platforms, `timespec` has anonymous padding fields, which"] # [doc = " Rust doesn't support yet (see `unnamed_fields`), so we define our own"] # [doc = " struct with explicit padding, with bidirectional `From` impls."] # [cfg (fix_y2038)] # [repr (C)] # [derive (Debug , Clone)] pub (crate) struct LibcTimespec { pub (crate) tv_sec : Secs , # [cfg (target_endian = "big")] padding : core :: mem :: MaybeUninit < u32 > , pub (crate) tv_nsec : i32 , # [cfg (target_endian = "little")] padding : core :: mem :: MaybeUninit < u32 > , }
    };
}

LibcTimespec!()